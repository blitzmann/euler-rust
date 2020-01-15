// from https://doc.rust-lang.org/rust-by-example/trait/iter.html
// tweaked to support a limit
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
