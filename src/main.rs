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

mod text; 

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
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Visual => write!(f, "VISUAL"),
        }
    }
}

impl Editor {
    fn new() -> Self {
        Editor { mode: Mode::Normal }
    }

    fn mode(&self) -> &Mode {
        &self.mode
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
                        }
                        (KeyCode::Char('v'), KeyModifiers::NONE) => {
                            self.mode = Mode::Visual;
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
                            if cursor_row >= h - 4 {
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
                        }
                        (KeyCode::Char('v'), KeyModifiers::NONE) => {
                            self.mode = Mode::Visual;
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
                        }
                        (KeyCode::Char('i'), KeyModifiers::NONE) => {
                            self.mode = Mode::Insert;
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
                    w = x; // problem is width and height is getting less after Resize...
                    h = y;
                    stdout.queue(SavePosition)?;
                    stdout.queue(Clear(ClearType::All))?;
                    print_tilde(&mut stdout, (w, h))?;
                    stdout.queue(RestorePosition)?;
                    if cursor_row >= y {
                        stdout.queue(MoveTo(cursor_col, y))?;
                    }
                    stdout.flush()?;
                }
                _ => {}
            }
        }
        stdout.queue(SavePosition)?;
        stdout.queue(MoveTo(0, h - 1))?; // Move to the bottom of the screen
        current_mode(&editor)?; // Print the current mode
        stdout.queue(RestorePosition)?;
        let (_,nh) = size()?;
        if cursor_row >= nh - 3 {
            stdout.queue(MoveTo(cursor_col, nh - 4))?;
        }
        stdout.flush()?;
    }
}

fn print_tilde(stdout: &mut std::io::Stdout, (_, h): (u16, u16)) -> Result<()> {
    let tilde = b"~";
    for i in 0..h - 3 {
        stdout.queue(MoveTo(0, i))?;
        stdout.write_all(tilde)?;
        let numbers = format!("{:>height$}", i + 1, height = 4);
        stdout.write(numbers.as_bytes())?;
        //print!("{} {} ", w, h);
    }
    stdout.flush()?;
    Ok(())
}

fn current_mode(editor: &Editor) -> Result<()> {
    let mode = editor.mode();
    print!("-- {mode} --");
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
