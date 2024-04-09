use crossterm::cursor::MoveTo;
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let tilde = b"~";
    let (_, h) = size().unwrap();

    enable_raw_mode().unwrap();
    stdout.queue(EnterAlternateScreen).unwrap();

    for i in 0..h {
        stdout.queue(MoveTo(0, i)).unwrap();
        stdout.write(tilde).unwrap();
    }
    stdout.queue(MoveTo(2, 0)).unwrap();
    stdout.flush().unwrap();
    sleep(Duration::from_secs(5));
    stdout.queue(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
