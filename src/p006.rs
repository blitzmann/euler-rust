
extern crate test;


pub fn solve(max: u32) -> u32 {
    (1..=max).sum::<u32>().pow(2) - (1..=max).map(|i| i).sum::<u32>()
}
