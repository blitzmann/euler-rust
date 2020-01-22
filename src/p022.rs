use std::env;
use std::fs;
use itertools::Itertools;

fn word_to_score (word: &str) -> u32 {
    word
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .map(|c| (c as u32) - 96)
        .sum()

}

pub fn solve() -> u32 {
    let contents = fs::read_to_string("src/p022_names.txt").expect("Something went wrong reading the file");
    contents
        .split(",")
        .map(|l| l.replace("\"", ""))
        // .inspect(|c| println!("{:?}", c))
        .sorted()
        .enumerate()
        // .inspect(|(i, n)| println!("{} {:?}", i, n))
        .map(|(i, n)| word_to_score(&n) * (i as u32 + 1))
        .sum()
    
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 648);
    }
}
