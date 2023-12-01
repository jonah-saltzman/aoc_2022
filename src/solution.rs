#[derive(Default)]
pub struct Calculator {
    board: Vec<Vec<u8>>,
    max_top: Vec<u8>,
    visible_trees: Vec<Vec<bool>>,
}

impl Calculator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handle_line(&mut self, line: Vec<u8>) {
        let mut max_left: u8 = 0;
        let mut max_right: u8 = 0;
        let len = line.len();

        if self.max_top.is_empty() {
            self.max_top = vec![0; len];
        }

        let mut visible_trees = vec![false; len];

        let mut i: usize = 0;
        while i < len {
            let max_top = self.max_top[i];
            let l_idx = i;
            let r_idx = len - i - 1;
            let left = line[l_idx];
            let right = line[r_idx];

            if left > max_left {
                visible_trees[l_idx] = true;
            }
            if right > max_right {
                visible_trees[r_idx] = true;
            }
            if left > max_top {
                visible_trees[l_idx] = true;
            }

            max_left = max_left.max(left);
            max_right = max_right.max(right);
            self.max_top[i] = self.max_top[i].max(left);

            i += 1;
        }
        self.visible_trees.push(visible_trees);
        self.board.push(line);
    }

    fn calculate_from_bottom(&mut self) {
        let mut max_bottom: Vec<u8> = vec![0; self.max_top.len()];
        for (y, line) in self.board.iter().enumerate().rev() {
            for (x, &n) in line.iter().enumerate() {
                if n > max_bottom[x] {
                    self.visible_trees[y][x] = true;
                }
                max_bottom[x] = max_bottom[x].max(n);
            }
        }
    }

    pub fn into_result(mut self) -> usize {
        self.calculate_from_bottom();
        self.visible_trees
            .into_iter()
            .flat_map(|line| line.into_iter())
            .filter(|&b| b)
            .count()
    }
}
