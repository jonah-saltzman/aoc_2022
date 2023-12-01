use std::io::{self, BufRead};

mod solution;

use solution::Calculator;

pub fn parse_line(line: &str) -> Vec<u8> {
    line.chars().map(|c| c as u8).collect()
}

fn main() {
    let mut buf = String::new();
    let mut reader = io::stdin().lock();
    let mut calc = Calculator::new();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let line = parse_line(&buf[..n - 1]);
        calc.handle_line(line);
        buf.clear();
    }
    let answer = calc.into_result();
    println!("{}", answer);
}
