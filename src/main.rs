use crossterm::cursor::MoveTo;
use crossterm::{
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    QueueableCommand,
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let tildr = b"~";
    let (_, h) = size().unwrap();

    enable_raw_mode().unwrap();
    stdout.queue(EnterAlternateScreen).unwrap();
    stdout.queue(MoveTo(0, 0)).unwrap();
    stdout.write(&tildr.repeat(h as usize)).unwrap();
    print!("{}", h);
    stdout.flush().unwrap();
    sleep(Duration::from_secs(5));
    stdout.queue(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
