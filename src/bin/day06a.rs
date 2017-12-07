// http://adventofcode.com/2017/day/6

extern crate unicode_segmentation;

use std::collections::HashSet;
use std::io;
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    let mut my_cursor = MyCursor::from_stdin();
    println!("{}", solve(&mut my_cursor));
}


fn solve(my_cursor: &mut MyCursor) -> usize {
    while my_cursor.steps_until_first_repeated_state == None {
        my_cursor.step();
    }
    my_cursor.steps_until_first_repeated_state.unwrap()
}


#[derive(Debug)]
struct MyCursor {
    // representation of memory banks
    banks: Vec<usize>,

    // number of times step() has been called on this struct
    steps: usize,

    // hashset of all previously seen states of the memory banks
    seen_states: HashSet<Vec<usize>>,

    // contains None until first repeated state is found and Some(steps_until_first_repeated_state)
    // afterward. N.B.: keeps first Some() value even if additional repeated states are found.
    steps_until_first_repeated_state: Option<usize>,
}

impl MyCursor {
    fn from_string(input: &str) -> MyCursor {
        MyCursor {
            banks: input.unicode_words().map(|w| w.parse().unwrap()).collect(),
            steps: 0,
            seen_states: HashSet::new(),
            steps_until_first_repeated_state: None,
        }
    }

    fn from_stdin() -> MyCursor {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        MyCursor::from_string(&input)
    }

    fn step(&mut self) {
        // update the step counter
        self.steps += 1;

        // find the index and value of the bank containing the most blocks
        let (max_index, max_value) = self.banks
            .iter()
            .enumerate()
            .fold(None, |max, curr_tuple| {
                let curr_i = curr_tuple.0;
                let curr_v = *curr_tuple.1;
                match max {
                    None => Some((curr_i, curr_v)),
                    Some((prev_i, prev_v)) => {
                        if curr_v > prev_v {
                            Some((curr_i, curr_v))
                        } else {
                            Some((prev_i, prev_v))
                        }
                    }
                }
            })
            .unwrap();

        // overwrite the bank at max_index to contain 0 blocks
        *(self.banks.get_mut(max_index).unwrap()) = 0;

        // distribute "max_value" blocks across all memory banks, starting with the bank
        // immediately after the one at max_index
        let length = self.banks.len();
        for i in 0..max_value {
            *(self.banks.get_mut((max_index + i + 1) % length).unwrap()) += 1;
        }

        // add the freshly mutated bank state to the set of seen states
        let insert_succeeded = self.seen_states.insert(self.banks.clone());

        // if our current state was already in the set of seen states AND the flag has a value of
        // None (i.e. this is the first time a repeated state has been encountered), record the
        // current number of steps in the flag
        if let (false, None) = (insert_succeeded, self.steps_until_first_repeated_state) {
            self.steps_until_first_repeated_state = Some(self.steps);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(&mut MyCursor::from_string("0 2 7 0")), 5);
    }
}
