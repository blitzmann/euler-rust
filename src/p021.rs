use itertools::Itertools;

fn d(n: u32) -> u32 {
    (1..((n as f64).sqrt() as u32) + 1)
        .step_by(if n % 2 == 1 { 2 } else { 1 }) // if it's an odd number, we can get rid of checking even numbers
        .filter(|i| n % i == 0)
        .flat_map(|i| std::iter::once(i).chain(std::iter::once(n / i)))
        .filter(|i| i < &n)
        .unique()
        // .collect::<Vec<u32>>()
        .sum()
}

pub fn solve() -> u32 {
    let mut p = 0;
    for a in 1..10000 {
        let b = d(a);
        let c = d(b);
        if  a!=b && a == c {
            // could add b as well, but then would have to exclude it from the iteration
            p += a;
        }
    }
    
    p
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 648);
    }
}
