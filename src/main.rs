use std::io::{self, BufRead, BufReader};

mod parser;
mod solution;

use parser::parse_instructions;
use solution::Calculator;

fn main() {
    let mut buf = String::with_capacity(30);
    let mut reader = BufReader::new(io::stdin().lock());
    let mut init_state: Vec<Vec<char>> = vec![];
    while let Ok(n) = reader.read_line(&mut buf) {
        if buf == "\n" {
            break;
        }
        let stack = parser::parse_init_state(&buf[..n - 1]);
        init_state.push(stack);
        buf.clear();
    }
    let mut calc = Calculator::new(init_state);
    buf.clear();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let cmd = parse_instructions(&buf[..n - 1]);
        calc.handle_command(cmd);
        buf.clear();
    }
    let ans: String = calc.get_answer().into_iter().collect();
    println!("{}", ans);
}
