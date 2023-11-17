use std::io::{self, Read, BufReader};

mod solution;

fn main() {
    let mut buf = [0u8; 4];
    let mut calc = solution::Calculator::default();
    let mut reader = BufReader::new(io::stdin().lock());
    while let Ok(_) = reader.read_exact(&mut buf) {
        calc.process_line(&buf);
    }
    let answer = calc.get_score();
    println!("{}", answer);
}
