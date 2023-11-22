use std::io::{self, BufRead, BufReader};

mod parser;
mod solution;

use parser::Parser;
use solution::Calculator;

fn main() {
    let mut buf = String::with_capacity(20);
    let mut calc = Calculator::default();
    let mut reader = BufReader::new(io::stdin().lock());
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let ranges = Parser::parse_line(&buf[..n - 1]);
        calc.process_pairs(ranges);
        buf.clear();
    }
    let answer = calc.get_score();
    println!("{}", answer);
}
