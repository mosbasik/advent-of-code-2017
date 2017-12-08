// http://adventofcode.com/2017/day/6

extern crate unicode_segmentation;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    let mut my_cursor = MyCursor::from_stdin();
    println!("{}", solve(&mut my_cursor));
}


fn solve(my_cursor: &mut MyCursor) -> usize {
    while my_cursor.first_repeat == None {
        my_cursor.step();
    }
    my_cursor.loop_size().unwrap()
}


#[derive(Clone, Debug, PartialEq)]
struct Bookmark {
    banks: Vec<usize>,
    steps: usize,
}


#[derive(Debug)]
struct MyCursor {
    // representation of memory banks
    banks: Vec<usize>,

    // number of times step() has been called on this struct
    steps: usize,

    // hashmap where the keys are all the previously seen states of the memory banks and the values
    // are the step counts when those states occured
    seen_states: HashMap<Vec<usize>, usize>,

    // contains None until the *second* occurance of any previously seen state is found, then Some
    // thereafter (containing a bookmark of the state and the number of steps taken to reach it
    // since the beginning of the program). Note: finding additional repeated states after the
    // first will not update this value.
    first_repeat: Option<Bookmark>,
}

impl MyCursor {
    fn from_string(input: &str) -> MyCursor {
        MyCursor {
            banks: input.unicode_words().map(|w| w.parse().unwrap()).collect(),
            steps: 0,
            seen_states: HashMap::new(),
            first_repeat: None,
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

        // try to insert the current (just updated) state to the map of seen states
        let insert_succeeded = match self.seen_states.entry(self.banks.clone()) {
            Entry::Occupied(..) => false,
            Entry::Vacant(e) => {
                e.insert(self.steps);
                true
            }
        };

        // if the insertion failed (because the current state's already been seen before) and no
        // first repeated state bookmark exists yet
        if !insert_succeeded && self.first_repeat == None {
            // bookmark the current state as the first repeated state
            self.first_repeat = Some(Bookmark {
                banks: self.banks.clone(),
                steps: self.steps,
            });
        }
    }

    fn loop_size(&self) -> Option<usize> {
        match self.first_repeat {
            None => None,
            Some(ref fr) => Some(fr.steps - self.seen_states.get(&(self.banks)).unwrap()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(&mut MyCursor::from_string("0 2 7 0")), 4);
    }
}
