use std::io;

mod solution;

fn main() {
    let mut buf = String::with_capacity(6);
    let mut calc = solution::Calculator::new();
    loop {
        match io::stdin().read_line(&mut buf) {
            Ok(0) => {
                calc.process_line("");
                break;
            },
            Ok(n) => {
                calc.process_line(&buf[..n - 1]);
                buf.clear();
            },
            Err(e) => panic!("{:?}", e)
        }
    }
    let answer = calc.get_max_calories();
    println!("{}", answer);
}
