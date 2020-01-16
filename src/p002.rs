// From https://doc.rust-lang.org/rust-by-example/trait/iter.html
// Tweaked to support a limit
pub struct Fibonacci {
    curr: u64,
    next: u64,
    stop: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        if self.curr >= self.stop {
            None
        } else {
            self.next = new_next;
            Some(self.curr)
        }
    }
}

pub fn fibonacci(stop: u64) -> Fibonacci {
    Fibonacci {
        curr: 1,
        next: 1,
        stop,
    }
}

pub fn solve(limit: u64) -> u64 {
    // todo: write test. limit of 100 = 44
    fibonacci(limit).filter(|i| i % 2 == 0).sum::<u64>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(89), 44);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(4_000_000), 4613732);
    }
}
