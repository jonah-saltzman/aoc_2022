pub struct Parser {}

impl Parser {
    pub fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
        let nums: Vec<i32> = line
            .split(',')
            .map(|s| s.split('-').map(|s| s.parse().unwrap()))
            .flatten()
            .collect();
        ((nums[0], nums[1]), (nums[2], nums[3]))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::Parser;

    #[test]
    fn parser_test() {
        let line: &str = "3-47,46-86";
        let parsed = Parser::parse_line(line);
        assert_eq!(parsed, ((3, 47), (46, 86)));
    }
}
