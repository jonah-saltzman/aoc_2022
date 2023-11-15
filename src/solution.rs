// input is a string with calorie counts on each line,
// split into groups with a double line break. Return
// the calorie count of the group with the most calories

use aoc_2022::MinHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Reverse<T>(T);

impl<T: PartialOrd> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Group,
    Break,
}

#[derive(Debug)]
pub struct Calculator {
    heap: MinHeap<Reverse<i32>>,
    curr_sum: i32,
    state: State,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            heap: MinHeap::new(),
            curr_sum: 0,
            state: State::Group,
        }
    }

    pub fn process_line(&mut self, line: &str) {
        match (self.state, line) {
            (State::Group, "") => {
                self.heap.push(Reverse(self.curr_sum));
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

    pub fn get_max_calories(&mut self) -> i32 {
        if self.state == State::Group {
            panic!("cannot interrupt calculation")
        }
        let mut sum = self.heap.pop().unwrap().0;
        sum += self.heap.pop().unwrap().0;
        sum += self.heap.pop().unwrap().0;
        sum
    }
}
