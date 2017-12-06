// http://adventofcode.com/2017/day/4

extern crate unicode_segmentation;

use std::collections::HashSet;
use std::io::{self, BufRead};
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    let stdin = io::stdin();
    let valid_count: i32 = stdin
        .lock()
        .lines()
        .map(|line| if is_valid(&line.unwrap()) { 1 } else { 0 })
        .sum();

    println!("{}", valid_count);
}


fn is_valid(line: &str) -> bool {
    let mut used_words = HashSet::new();
    let collisions: HashSet<bool> = line.unicode_words().map(|w| used_words.insert(w)).collect();
    !collisions.contains(&false)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("aa bb cc dd ee"), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid("aa bb cc dd aa"), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid("aa bb cc dd aaa"), true);
    }
}
