use std::collections::HashSet;

fn get_priority(item: u8) -> i32 {
    match item {
        b'a'..=b'z' => (item - 96).into(),
        b'A'..=b'Z' => (item - 64 + 26).into(),
        _ => unreachable!(),
    }
}

#[derive(Default)]
pub struct Calculator {
    score: i32,
    items: HashSet<u8>,
    state: usize,
}

impl Calculator {
    pub fn process_line(&mut self, line: &[u8]) {
        let new_set: HashSet<u8> = if self.state == 0 {
            line.iter().map(|&e| e).collect()
        } else {
            line.iter()
                .filter_map(|e| self.items.get(e).map(|&e| e))
                .collect()
        };
        if self.state == 2 {
            let item = *new_set.iter().next().unwrap();
            let priority = get_priority(item);
            self.score += priority;
            self.items = HashSet::with_capacity(52);
            self.state = 0;
        } else {
            self.items = new_set;
            self.state += 1;
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}
