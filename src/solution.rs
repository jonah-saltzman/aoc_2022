use std::collections::HashSet;

fn get_priority(item: u8) -> i32 {
    match item {
        b'a'..=b'z' => (item - 96).into(),
        b'A'..=b'Z' => (item - 64 + 26).into(),
        _ => unreachable!()
    }
}

#[derive(Default)]
pub struct Calculator {
    score: i32,
    items: HashSet<u8>
}

impl Calculator {
    pub fn process_line(&mut self, line: &[u8]) {
        let mid = line.len() / 2;
        for &item in &line[..mid] {
            self.items.insert(item);
        }
        let val = 'val: {
            for item in &line[mid..] {
                if self.items.contains(item) {
                    break 'val *item
                }
            }
            unreachable!()
        };
        let priority = get_priority(val);
        self.score += priority;
        self.items.clear();
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}
