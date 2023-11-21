use std::io::{self, BufReader, BufRead};

mod solution;

fn main() {
    let mut buf = Vec::with_capacity(60);
    let mut calc = solution::Calculator::default();
    let mut reader = BufReader::new(io::stdin().lock());
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n == 0 { break }
        calc.process_line(&buf[0..n - 1]);
        buf.clear();
    }
    let answer = calc.get_score();
    println!("{}", answer);
}
