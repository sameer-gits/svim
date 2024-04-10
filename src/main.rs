use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use std::io::{stdout, Result, Write};

fn main() -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.queue(EnterAlternateScreen)?;
    print_tilde(&mut stdout)?;
    loop {
        match read_char()? {
            'q' => {
                quit_svim(&mut stdout)?;
                break;
            }

            'h' => {
                stdout.queue(MoveLeft(1))?;
                stdout.flush()?;
            }

            'j' => {
                stdout.queue(MoveDown(1))?;
                stdout.flush()?;
            }

            'k' => {
                stdout.queue(MoveUp(1))?;
                stdout.flush()?;
            }

            'l' => {
                stdout.queue(MoveRight(1))?;
                stdout.flush()?;
            }
            _ => {}
        };
    }

    Ok(())
}

fn quit_svim(stdout: &mut std::io::Stdout) -> Result<()> {
    stdout.queue(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: event::KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn print_tilde(stdout: &mut std::io::Stdout) -> Result<()> {
    let tilde = b"~";
    let (_, h) = size()?;

    for i in 0..h - 2 {
        stdout.queue(MoveTo(0, i))?;
        stdout.write_all(tilde)?;
        print!("{}", h);
    }
    stdout.queue(MoveTo(4, 0))?;
    stdout.flush()?;

    Ok(())
}
