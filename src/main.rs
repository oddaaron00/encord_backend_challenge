use std::io::{self, BufRead, Write};

use tetris_exercise::game::output_height;

fn main() {
    run();
}

fn run() {
    let mut stdout = io::stdout().lock();

    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| output_height(line, stdout.by_ref()));

    stdout.write_all(b"\n").unwrap();
    stdout.flush().unwrap();
}
