use crossterm::cursor::MoveTo;
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let tildr = b"~";
    let (_, h) = terminal::size().unwrap();

    terminal::enable_raw_mode().unwrap();
    stdout.queue(EnterAlternateScreen).unwrap();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.write(&tildr.repeat(h as usize)).unwrap();
    print!("{}", h);
    stdout.flush().unwrap();
    thread::sleep(Duration::from_secs(5));
    stdout.queue(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
