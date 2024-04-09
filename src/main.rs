use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use std::io::{stdout, Write, Result};

fn main() -> Result<()> {
    let mut stdout = stdout();
    let tilde = b"~";
    let (_, h) = size()?;

    enable_raw_mode()?;
    stdout.queue(EnterAlternateScreen)?;

    for i in 0..h {
        stdout.queue(MoveTo(0, i))?;
        stdout.write_all(tilde)?;
    }
    stdout.queue(MoveTo(2, 0))?;
    stdout.flush()?;

    loop {
        match read_char()? {
            'q' => {
                quit_svim(&mut stdout)?;
                break;
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
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
