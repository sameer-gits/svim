use crossterm::cursor::{
    MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition,
};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use std::io::{stdout, Result, Write};
use std::time::Duration;

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.queue(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let (mut w, mut h) = size()?;
    print_tilde(&mut stdout, (w, h))?;
    stdout.queue(MoveTo(4, 0))?;
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
                        stdout.queue(MoveLeft(1))?;
                        stdout.flush()?;
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
        print!("{}", i + 1);
        //print!("{} {} ", w, h);
    }
    stdout.queue(RestorePosition)?;
    stdout.flush()?;
    Ok(())
}
