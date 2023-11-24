use std::io::{self, Read};

mod solution;

use solution::Calculator;

fn main() {
    let mut buf: [u8; 4096] = [0; 4096];
    let mut reader = io::stdin().lock();
    let mut calc = Calculator::new();
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            println!("EOF without answer");
            break;
        }
        if let Some(ans) = calc.process_chars(&buf[..n]) {
            println!("{}", ans);
            break
        }
    }
}
