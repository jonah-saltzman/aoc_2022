use std::io::{self, BufRead};

mod solution;
mod parser;

use solution::Calculator;
use parser::Parser;

fn main() {
    let mut buf = String::new();
    let mut reader = io::stdin().lock();
    let mut calc = Calculator::new();
    let mut parser = Parser::new();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            if let Some(group) = parser.end() {
                calc.handle_group(group);
            }
            break
        }
        for group in parser.line(&buf[..n - 1]) {
            calc.handle_group(group);
        }
        buf.clear();
    }
    let answer = calc.get_result();
    println!("{}", answer);
}
