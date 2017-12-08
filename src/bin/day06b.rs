// http://adventofcode.com/2017/day/6

extern crate unicode_segmentation;

use std::collections::HashSet;
use std::io;
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    let mut my_cursor = MyCursor::from_stdin();
    println!("{}", solve(&mut my_cursor));
}


fn solve(c: &mut MyCursor) -> usize {
    while c.first_loop == None {
        c.step();
    }
    c.loop_size().unwrap()
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

    // hashset of all previously seen states of the memory banks
    seen_states: HashSet<Vec<usize>>,

    // contains None until the *second* occurance of any previously seen state is found, then Some
    // thereafter (containing a bookmark of the state and the number of steps taken to reach it).
    // Note: finding additional repeated states after the first will not update this value.
    first_repeat: Option<Bookmark>,

    // contains None until the *second* occurance of the state in first_repeat is found, then Some
    // thereafter (containing a bookmark of the state and the number of steps taken to reach it).
    // Note: finding additional occurances of the state in first_repeat after the second will not
    // update this value.
    first_loop: Option<Bookmark>,
}

impl MyCursor {
    fn from_string(input: &str) -> MyCursor {
        let banks = input.unicode_words().map(|w| w.parse().unwrap()).collect();
        // println!("{} steps\t{:?}", 0, banks);
        MyCursor {
            banks,
            steps: 0,
            seen_states: HashSet::new(),
            first_repeat: None,
            first_loop: None,
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

        // try to add the current (just updated) state to the set of seen states
        let insert_succeeded = self.seen_states.insert(self.banks.clone());

        // if that fails because the current state's already been seen before
        if !insert_succeeded {

            // check if a first repeat state has been bookmarked yet
            match self.first_repeat {
                None => {
                    // if not, bookmark the current state as the first repeated state
                    self.first_repeat = Some(Bookmark {
                        banks: self.banks.clone(),
                        steps: self.steps,
                    });
                }
                Some(ref first_repeat) => {
                    // if so, and if the current state matches the first repeat state, and if a
                    // first loop bookmark doesn't already exist
                    if self.banks == first_repeat.banks && self.first_loop == None {
                        // bookmark the current state as the first looped state
                        self.first_loop = Some(Bookmark {
                            banks: self.banks.clone(),
                            steps: self.steps,
                        });
                    }
                }
            };
        }

        // println!("{} steps\t{:?}", self.steps, self.banks);
    }

    fn loop_size(&self) -> Option<usize> {
        if let Some(ref first_loop) = self.first_loop {
            if let Some(ref first_repeat) = self.first_repeat {
                return Some(first_loop.steps - first_repeat.steps);
            }
        }
        None
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
