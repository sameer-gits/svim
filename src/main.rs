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

fn print_tilde(stdout: &mut std::io::Stdout, (w, h): (u16, u16)) -> Result<()> {
    let tilde = b"~";
    let intro = b"This is SVIM v0.0.1";
    stdout.queue(SavePosition)?;

    for i in 0..h - 2 {
        stdout.queue(MoveTo(0, i))?;
        stdout.write_all(tilde)?;
        let numbers = format!("{:>height$}", i + 1, height = 3);
        print!("{}", numbers);
        //print!("{} {} ", w, h);
    }
    stdout.queue(MoveTo(w / 2 - intro.len() as u16 / 2 + 2, h / 2))?;
    stdout.write_all(intro)?;
    stdout.queue(RestorePosition)?;
    stdout.flush()?;
    Ok(())
}
