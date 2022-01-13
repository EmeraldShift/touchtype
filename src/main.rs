use rand::Rng;
use std::io::Read;
use std::io::{self, Write};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

static HOME_ROW: &str = "asdfghjkl;";

fn main() {
    let stdin = 0;
    let termios = Termios::from_fd(0).unwrap();

    // Setup terminal to read in raw chars
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO); // No echo or canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let mut r = rand::thread_rng();
    let mut score = 0;
    loop {
        // Pick a random character on the home row
        let i: usize = r.gen_range(0..HOME_ROW.len());
        let c = HOME_ROW.bytes().nth(i).unwrap();

        // Request the user to type it
        print!("\r{}{}", c as char, 7 as char); // 7 == bell character
        io::stdout().flush().unwrap();

        let mut buf = [0; 1];
        io::stdin().read_exact(&mut buf).unwrap();

        if buf[0] != c {
            println!("\rFAIL! Your score was: {}", score);
            break;
        }

        score += 1;
    }

    // Reset terminal settings
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
