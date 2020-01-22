use itertools::Itertools;

pub fn solve(n: u32) -> u32 {
    let mut i = 1;

    loop {
        let triangle = (i * (i + 1)) / 2;
        let factors = factors(triangle).len();
        if factors as u32 > n {
            return triangle;
        }
        i += 1;
    }
}

fn factors(n: u32) -> Vec<u32> {
    (1..=((n as f64).sqrt() as u32) + 1)
        .step_by(if n % 2 == 1 { 2 } else { 1 }) // if it's an odd number, we can get rid of checking even numbers
        .filter(|i| n % i == 0)
        .flat_map(|i| std::iter::once(i).chain(std::iter::once(n / i)))
        .unique()
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(5), 28);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(500), 76_576_500);
    }
}
