use std::collections::VecDeque;

macro_rules! to_idx {
    ($i:expr) => {
        ($i - 97) as usize
    };
}

pub struct Calculator {
    window: VecDeque<u8>,
    counts: [u8; 26],
    repeats: u8,
    idx: usize,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            window: VecDeque::new(),
            counts: [0; 26],
            repeats: 0,
            idx: 0,
        }
    }

    fn push(&mut self, c: u8) {
        let initial = self.counts[to_idx!(c)];
        if initial == 1 {
            self.repeats += 1;
        }
        self.counts[to_idx!(c)] = initial + 1;
        self.window.push_back(c);
        self.idx += 1;
    }

    fn pop(&mut self) {
        let c = self.window.pop_front().unwrap();
        let initial = self.counts[to_idx!(c)];
        if initial == 2 {
            self.repeats -= 1;
        }
        self.counts[to_idx!(c)] = initial - 1;
    }

    pub fn process_chars(&mut self, chars: &[u8]) -> Option<usize> {
        for &c in chars {
            self.push(c);
            if self.idx <= 14 {
                if self.is_finished() {
                    return Some(self.idx);
                }
                continue;
            }
            self.pop();
            if self.is_finished() {
                return Some(self.idx);
            }
        }
        None
    }

    pub fn is_finished(&self) -> bool {
        self.repeats == 0 && self.idx >= 14
    }
}
