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
        i+=1;
    }
    r
}
