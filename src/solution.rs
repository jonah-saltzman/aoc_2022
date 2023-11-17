const fn win_move(c: u8) -> u8 {
    match c {
        b'A' => b'Y',
        b'B' => b'Z',
        b'C' => b'X',
        _ => unreachable!()
    }
}

const fn move_score(c: u8) -> i32 {
    match c {
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => unreachable!()
    }
}

#[derive(Default)]
pub struct Calculator {
    score: i32
}

impl Calculator {
    pub fn process_line(&mut self, line: &[u8; 4]) {
        let opponent: u8 = line[0];
        let player: u8 = line[2];
        let is_win: bool = win_move(opponent) == player;
        let game_score = if is_win { 6 } else if (player - 23) == opponent { 3 } else { 0 };
        let move_score = move_score(player);
        self.score += game_score + move_score;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}