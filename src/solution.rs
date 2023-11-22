#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Command {
    pub qty: usize,
    pub src: usize,
    pub dst: usize,
}

pub struct Calculator {
    state: Vec<Vec<char>>,
}

impl Calculator {
    pub fn new(init: Vec<Vec<char>>) -> Self {
        Self { state: init }
    }

    pub fn handle_command(&mut self, cmd: Command) {
        let split_at = self.state[cmd.src - 1].len() - cmd.qty;
        let moved = self.state[cmd.src - 1].split_off(split_at);
        self.state[cmd.dst - 1].extend(moved);
    }

    pub fn get_answer(self) -> Vec<char> {
        let mut answer = vec![];
        for mut stack in self.state.into_iter() {
            answer.push(stack.pop().unwrap());
        }
        answer
    }
}
