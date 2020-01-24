use itertools::Itertools;
use std::collections::HashSet;

fn factors(n: u32) -> u32 {
    (1..((n as f64).sqrt() as u32) + 1)
        .step_by(if n % 2 == 1 { 2 } else { 1 }) // if it's an odd number, we can get rid of checking even numbers
        .filter(|i| n % i == 0)
        .flat_map(|i| std::iter::once(i).chain(std::iter::once(n / i)))
        .filter(|i| i < &n)
        .unique()        
        .sum()
}


pub fn solve() -> u32 {
    let sum_of_abundants: HashSet<u32> = (1..=28123)
        .filter(|i| factors(*i) > *i)
        .combinations(2)
        .into_iter()
        .map(|vec| vec.into_iter().sum())
        .unique()
        .collect();

    (1..=28123)
        .filter(|i| sum_of_abundants.contains(i))
        .inspect(|x|println!("{}",x ))
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
