// input is a string with calorie counts on each line,
// split into groups with a double line break. Return
// the calorie count of the group with the most calories

#[derive(Copy, Clone, PartialEq)]
enum State {
    Group,
    Break
}

impl Default for State {
    fn default() -> Self {
        State::Group
    }
}


#[derive(Default)]
pub struct Calculator {
    max: i32,
    curr_sum: i32,
    state: State
}

impl Calculator {
    pub fn new() -> Self {
        Self::default()
    }
  
    pub fn process_line(&mut self, line: &str) {
        match (self.state, line) {
            (State::Group, "") => {
                self.max = self.max.max(self.curr_sum);
                self.state = State::Break;
            },
            (State::Group, s) => {
                let calories: i32 = s.parse().unwrap();
                self.curr_sum += calories;
            },
            (State::Break, "") => panic!("invalid state"),
            (State::Break, s) => {
                let calories: i32 = s.parse().unwrap();
                self.curr_sum = calories;
                self.state = State::Group;
            },
        }
    }

    pub fn get_max_calories(&self) -> i32 {
        if self.state == State::Group {
            panic!("cannot interrupt calculation")
        }
        self.max
    }
}