// http://adventofcode.com/2017/day/1

extern crate unicode_segmentation;

use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // read puzzle input as single line from stdin
    let mut puzzle_input = String::new();
    io::stdin().read_line(&mut puzzle_input).unwrap();

    // parse puzzle input string to vector of u32
    let digits: Vec<u32> = puzzle_input
        .trim()
        .graphemes(true)
        .map(|g| g.parse().unwrap())
        .collect();

    // initialize "previous" digit to the last digit of the vector (because it's a ring)
    let mut prev_d = digits.iter().rev().next().unwrap();

    let mut sum = 0;
    for d in digits.iter() {
        if d == prev_d {
            sum += d;
        }
        prev_d = d;
    }

    println!("{}", sum);
}
