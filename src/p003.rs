/*
    The solution to this problem is fairly straightforward. We continue looping
    through an iteration, checking if goal is evenly divisible by i. If it is, i
    is a prime factor - store it (it will automatically be the largest as we only
    increase i). From here, continue by setting goal = goal / i
*/

pub fn solve(mut goal: u64) -> u64 {
    // todo: write test. goal of 13195 = 29
    let mut i = 2;
    let mut r = 0;
    loop {
        if goal % i == 0 {
            r = i;
            goal /= i;
            if goal == 1 {
                break;
            }
        }
        i += 1;
    }
    r
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(13195), 29);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(600_851_475_143), 6857);
    }
}
