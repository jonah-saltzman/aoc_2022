use crate::solution::Command;

pub fn parse_init_state(line: &str) -> Vec<char> {
    line.chars().collect()
}

pub fn parse_instructions(line: &str) -> Command {
    let mut tokens = line.split(' ');
    let qty: usize = tokens.nth(1).unwrap().parse().unwrap();
    let src: usize = tokens.nth(1).unwrap().parse().unwrap();
    let dst: usize = tokens.nth(1).unwrap().parse().unwrap();
    Command { qty, src, dst }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init_test() {
        let input = "MCD";
        let output = parse_init_state(input);
        assert_eq!(output, vec!['M', 'C', 'D'])
    }

    #[test]
    fn cmd_test() {
        let input = "move 3 from 1 to 2";
        let cmd = parse_instructions(input);
        assert_eq!(
            cmd,
            Command {
                qty: 3,
                src: 1,
                dst: 2
            }
        );
    }
}
