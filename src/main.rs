use crossterm::cursor::{
    position, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition,
};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use std::fmt::{self, Display, Formatter};
use std::io::{stdout, Result, Write};
use std::time::Duration;

enum Mode {
    Normal,
    Insert,
    Visual,
}

struct Editor {
    mode: Mode,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Insert => write!(f, "Insert"),
            Mode::Visual => write!(f, "Visual"),
        }
    }
}

impl Editor {
    fn new() -> Self {
        Editor { mode: Mode::Normal }
    }

    fn switch_mode(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Normal => match key_event.code {
                KeyCode::Char('v') => {
                    self.mode = Mode::Visual;
                    println!("Switched to {} mode", self.mode);
                }
                KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                    println!("Switched to {} mode", self.mode);
                }
                _ => {}
            },
            Mode::Insert => match key_event.code {
                KeyCode::Char('v') => {
                    self.mode = Mode::Visual;
                    println!("Switched to {} mode", self.mode);
                }
                KeyCode::Char('n') => {
                    self.mode = Mode::Normal;
                    println!("Switched to Visual mode");
                }
                _ => {}
            },
            Mode::Visual => match key_event.code {
                KeyCode::Char('n') => {
                    self.mode = Mode::Normal;
                    println!("Switched to {} mode", self.mode);
                }
                KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                    println!("Switched to {} mode", self.mode);
                }
                _ => {}
            },
        }
    }
}

fn main() -> Result<()> {
    let mut editor = Editor::new();
    loop {
        if let Event::Key(key_event) = read()? {
            editor.switch_mode(key_event);
        }
    }

    let mut stdout = stdout();
    stdout.queue(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let (mut w, mut h) = size()?;
    print_tilde(&mut stdout, (w, h))?;
    print_intro(&mut stdout, (w, h))?;
    stdout.queue(MoveTo(6, 0))?;
    stdout.flush()?;

    loop {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Resize(x, y) => {
                    w = x;
                    h = y;
                    stdout.queue(Clear(ClearType::All))?;
                    print_tilde(&mut stdout, (w, h))?;
                    stdout.flush()?;
                }
                Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    if let KeyCode::Char('q') = code {
                        disable_raw_mode()?;
                        stdout.queue(LeaveAlternateScreen)?;
                        return Ok(());
                    }
                }
                Event::Key(KeyEvent { code, .. }) => {
                    if let KeyCode::Char('h') = code {
                        let (cursor_col, _) = position()?;
                        if cursor_col <= 6 {
                            //do nothing!
                        } else {
                            stdout.queue(MoveLeft(1))?;
                            stdout.flush()?;
                        }
                    }
                    if let KeyCode::Char('j') = code {
                        stdout.queue(MoveDown(1))?;
                        stdout.flush()?;
                    }
                    if let KeyCode::Char('k') = code {
                        stdout.queue(MoveUp(1))?;
                        stdout.flush()?;
                    }
                    if let KeyCode::Char('l') = code {
                        stdout.queue(MoveRight(1))?;
                        stdout.flush()?;
                    }
                }
                _ => {}
            }
        }
    }
}

fn print_tilde(stdout: &mut std::io::Stdout, (_, h): (u16, u16)) -> Result<()> {
    let tilde = b"~";
    stdout.queue(SavePosition)?;

    for i in 0..h - 2 {
        stdout.queue(MoveTo(0, i))?;
        stdout.write_all(tilde)?;
        let numbers = format!("{:>height$}", i + 1, height = 4);
        print!("{}", numbers);
        //print!("{} {} ", w, h);
    }
    stdout.queue(RestorePosition)?;
    stdout.flush()?;
    Ok(())
}

fn print_intro(stdout: &mut std::io::Stdout, (w, h): (u16, u16)) -> Result<()> {
    let intro = b"This is SVIM v0.0.1";
    stdout.queue(SavePosition)?;
    stdout.queue(MoveTo(w / 2 - intro.len() as u16 / 2 + 2, h / 2))?;
    stdout.write_all(intro)?;
    stdout.flush()?;
    stdout.queue(RestorePosition)?;
    Ok(())
}
