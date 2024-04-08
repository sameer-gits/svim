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

    //let (w, h) = terminal::size().unwrap();

    terminal::enable_raw_mode().unwrap();
    stdout.queue(EnterAlternateScreen).unwrap();
    stdout.queue(MoveTo(5, 5)).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_secs(5));
    stdout.queue(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
