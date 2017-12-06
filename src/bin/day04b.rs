// http://adventofcode.com/2017/day/4

extern crate unicode_segmentation;

use std::collections::{BTreeSet, BTreeMap};
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
    let mut used_indexes = BTreeSet::new();
    let collisions: BTreeSet<bool> = line.unicode_words()
        .map(|w| used_indexes.insert(create_index_from(w)))
        .collect();
    !collisions.contains(&false)
}


fn create_index_from(word: &str) -> BTreeMap<&str, i32> {
    let mut index = BTreeMap::new();
    for g in word.graphemes(true) {
        *(index.entry(g).or_insert(0)) += 1;
    }
    index
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(is_valid("abcde fghij"), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_valid("abcde xyz ecdab"), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_valid("a ab abc abd abf abj"), true);
    }

    #[test]
    fn test_4() {
        assert_eq!(is_valid("iiii oiii ooii oooi oooo"), true);
    }

    #[test]
    fn test_5() {
        assert_eq!(is_valid("oiii ioii iioi iiio"), false);
    }
}
