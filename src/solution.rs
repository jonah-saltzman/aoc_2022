// input is a string with calorie counts on each line,
// split into groups with a double line break. Return
// the calorie count of the group with the most calories

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Group,
    Break,
}

#[derive(Debug)]
pub struct Calculator {
    heap: Vec<i32>,
    curr_sum: i32,
    state: State,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            heap: Vec::with_capacity(3),
            curr_sum: 0,
            state: State::Group,
        }
    }

    pub fn process_line(&mut self, line: &str) {
        match (self.state, line) {
            (State::Group, "") => {
                if self.heap.len() < 3 {
                    self.heap.push(self.curr_sum);
                } else {
                    if self.curr_sum > self.heap[0] {
                        self.heap[0] = self.curr_sum;
                    }
                    self.heap.sort_unstable();
                }
                self.state = State::Break;
            }
            (State::Group, s) => {
                let calories: i32 = s.parse().unwrap();
                self.curr_sum += calories;
            }
            (State::Break, "") => panic!("invalid state"),
            (State::Break, s) => {
                let calories: i32 = s.parse().unwrap();
                self.curr_sum = calories;
                self.state = State::Group;
            }
        }
    }

    pub fn get_max_calories(&self) -> i32 {
        if self.state == State::Group {
            panic!("cannot interrupt calculation")
        }
        self.heap.iter().sum()
    }
}
