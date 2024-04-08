use crossterm::cursor::MoveTo;
use crossterm::{
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(Clear(ClearType::Purge)).unwrap();
    stdout.queue(MoveTo(5, 5)).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_secs(5));
    terminal::disable_raw_mode().unwrap();
}
