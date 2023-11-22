#[derive(Default)]
pub struct Calculator {
    score: i32,
}

impl Calculator {
    pub fn process_pairs(&mut self, pairs: ((i32, i32), (i32, i32))) {
        let (higher, lower) = if pairs.0 .0 > pairs.1 .0 {
            (pairs.0, pairs.1)
        } else {
            (pairs.1, pairs.0)
        };
        if higher.0 <= lower.1 {
            self.score += 1;
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}
