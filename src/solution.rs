const fn win_move(c: u8) -> u8 {
    match c {
        b'A' => b'Y',
        b'B' => b'Z',
        b'C' => b'X',
        _ => unreachable!(),
    }
}

const fn lose_move(c: u8) -> u8 {
    match c {
        b'A' => b'Z',
        b'B' => b'X',
        b'C' => b'Y',
        _ => unreachable!(),
    }
}

const fn move_score(c: u8) -> i32 {
    match c {
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => unreachable!(),
    }
}

#[derive(Default)]
pub struct Calculator {
    score: i32,
}

impl Calculator {
    pub fn process_line(&mut self, line: &[u8; 4]) {
        let opponent: u8 = line[0];
        let outcome: u8 = line[2];
        let (choice, score): (u8, i32) = if outcome == 88 {
            (lose_move(opponent), 0)
        } else if outcome == 89 {
            (opponent + 23, 3)
        } else {
            (win_move(opponent), 6)
        };
        let total_score = score + move_score(choice);
        self.score += total_score;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}
