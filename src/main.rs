use crossterm::cursor::{
    position, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition,
};
use crossterm::event::{self, poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
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

    fn switch_mode(&mut self, event: Event, stdout: &mut std::io::Stdout) -> Result<bool> {
        match event {
            Event::Key(KeyEvent { code, modifiers, .. }) => {
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
                        // Handle other key events for Normal mode
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
    stdout.flush()?;
    loop {
        if let Ok(event) = read() {
            if editor.switch_mode(event, &mut stdout)? {
                break; // Exit the loop if switch_mode returns true
            }
        }
    }
    Ok(())
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
