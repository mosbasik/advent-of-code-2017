// http://adventofcode.com/2017/day/5

use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let instructions: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    println!("{}", solve(&mut MyCursor::new(instructions)));
}


fn solve(my_cursor: &mut MyCursor) -> u32 {
    while !my_cursor.next() {}
    my_cursor.counter
}


#[derive(Debug, PartialEq)]
struct MyCursor {
    index: i32, // index of current instruction
    counter: u32, // number of instructions consumed so far
    instructions: Vec<i32>, // instructions to follow
}

impl MyCursor {
    fn new(instructions: Vec<i32>) -> MyCursor {
        MyCursor {
            index: 0,
            counter: 0,
            instructions,
        }
    }

    fn next(&mut self) -> bool {

        // increment internal step counter
        self.counter += 1;

        let instruction = match self.instructions.get_mut(self.index as usize) {
            Some(i) => {
                let delta = match *i >= 3 {
                    true => -1, // decrement if offset is 3 or more
                    false => 1, // increment otherwise
                };
                *i += delta; // modify the instruction in the vector
                *i - delta // but pass out its original value to be used
            }
            None => panic!("shouldn't get here"),
        };

        // compute new position using instruction
        let new_index = self.index + instruction;

        // if new position is out of bounds, we've escaped!
        if new_index < 0 || self.instructions.len() as i32 <= new_index {
            return true;
        }

        // move to the new position
        self.index = new_index;

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut my_cursor = MyCursor {
            index: 0,
            counter: 0,
            instructions: vec![0, 3, 0, 1, -3],
        };

        assert_eq!(solve(&mut my_cursor), 10);
    }
}
