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

    fn switch_mode(
        &mut self,
        event: Event,
        stdout: &mut std::io::Stdout,
        (_, h): (u16, u16),
        (cursor_col, cursor_row): (u16, u16),
    ) -> Result<bool> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                if code == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                    disable_raw_mode()?;
                    stdout.queue(LeaveAlternateScreen)?;
                    stdout.flush()?;
                    return Ok(true); // Signal to break the loop
                }
            }
            _ => {}
        }

        match self.mode {
            Mode::Normal => match event {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => {
                    match (code, modifiers) {
                        (KeyCode::Char('i'), KeyModifiers::NONE) => {
                            self.mode = Mode::Insert;
                            println!("Switched to {} mode", self.mode);
                        }
                        (KeyCode::Char('v'), KeyModifiers::NONE) => {
                            self.mode = Mode::Visual;
                            println!("Switched to {} mode", self.mode);
                        }
                        //  Below are movement keys..
                        (KeyCode::Char('h'), KeyModifiers::NONE) => {
                            if cursor_col <= 6 {
                                //do nothing!
                            } else {
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;
                            }
                        }
                        (KeyCode::Char('j'), KeyModifiers::NONE) => {
                            if cursor_row >= h - 3 {
                                //do nothing!
                            } else {
                                stdout.queue(MoveDown(1))?;
                                stdout.flush()?;
                            }
                        }
                        (KeyCode::Char('k'), KeyModifiers::NONE) => {
                            stdout.queue(MoveUp(1))?;
                            stdout.flush()?;
                        }
                        (KeyCode::Char('l'), KeyModifiers::NONE) => {
                            stdout.queue(MoveRight(1))?;
                            stdout.flush()?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            },

            Mode::Insert => match event {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => {
                    match (code, modifiers) {
                        (KeyCode::Char('n'), KeyModifiers::NONE) => {
                            self.mode = Mode::Normal;
                            println!("Switched to {} mode", self.mode);
                        }
                        (KeyCode::Char('v'), KeyModifiers::NONE) => {
                            self.mode = Mode::Visual;
                            println!("Switched to {} mode", self.mode);
                        }
                        // Handle other key events for Insert mode
                        _ => {}
                    }
                }
                _ => {}
            },

            Mode::Visual => match event {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => {
                    match (code, modifiers) {
                        (KeyCode::Char('n'), KeyModifiers::NONE) => {
                            self.mode = Mode::Normal;
                            println!("Switched to {} mode", self.mode);
                        }
                        (KeyCode::Char('i'), KeyModifiers::NONE) => {
                            self.mode = Mode::Insert;
                            println!("Switched to {} mode", self.mode);
                        }
                        // Handle other key events for Visual mode
                        _ => {}
                    }
                }
                _ => {}
            },
        }
        Ok(false)
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut editor = Editor::new();
    stdout.queue(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let (mut w, mut h) = size()?;
    print_tilde(&mut stdout, (w, h))?;
    print_intro(&mut stdout, (w, h))?;
    stdout.queue(MoveTo(6, 0))?;
    stdout.flush()?;

    loop {
        let (cursor_col, cursor_row) = position()?;
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => {
                    if editor.switch_mode(
                        Event::Key(event),
                        &mut stdout,
                        (w, h),
                        (cursor_col, cursor_row),
                    )? {
                        return Ok(());
                    }
                }
                Event::Resize(x, y) => {
                    w = x;
                    h = y;
                    stdout.queue(SavePosition)?;
                    stdout.queue(Clear(ClearType::All))?;
                    print_tilde(&mut stdout, (w, h))?;
                    stdout.queue(RestorePosition)?;
                    stdout.flush()?;
                }
                _ => {}
            }
        }
        if cursor_row >= h - 2 {
            stdout.queue(MoveTo(cursor_col, h - 2))?;
        }
    }
}

fn print_tilde(stdout: &mut std::io::Stdout, (_, h): (u16, u16)) -> Result<()> {
    let tilde = b"~";

    for i in 0..h - 2 {
        stdout.queue(MoveTo(0, i))?;
        stdout.write_all(tilde)?;
        let numbers = format!("{:>height$}", i + 1, height = 4);
        stdout.write(numbers.as_bytes())?;
        //print!("{} {} ", w, h);
    }
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
