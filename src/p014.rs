use std::collections::HashMap;



fn slow() -> u64 {
    // let mut r = Box::new(0);

    fn collatz_wo_cache(n: u64) -> u64 {
        // *r = *r + 1;
        if n == 1 {
            1
        } else if n % 2 == 0 {
            1 + collatz_wo_cache(n / 2)
        } else {
            1 + collatz_wo_cache((3 * n + 1) / 2) // /2 is an optimization that is available, can test via bench
        }
}
    // collatz(13)
    let t = (1..1_000_000u64)
        .map(|i| (i, collatz_wo_cache(i)))
        // .count() as u32
        .max_by_key(|x| x.1)
        .unwrap()
        .0;

        // println!("{:?}", r); // 88_826_376
    t
}


fn original() -> u64 {

    fn collatz(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        // *r = *r + 1;
        let t = cache.get(&n);
        let s = format!("{:?}", t);
        match t {
            Some(result) => *result,
            None => {
                let result = if n == 1 {
                    1
                } else if n % 2 == 0 {
                    1 + collatz(n / 2, cache, )
                } else {
                    1 + collatz((3 * n + 1) / 2, cache) // /2 is an optimization that is available, can test via bench
                };
                cache.insert(n, result);
                result
            }
        } 
    }

    let mut collatz_cache: HashMap<u64, u64> = HashMap::new();

    // collatz(13)
    let t = (1..1_000_000u64)
        .map(|i| (i, collatz(i, &mut collatz_cache)))
        // .count() as u32
        .max_by_key(|x| x.1)
        .unwrap()
        .0;
    // println!("{:?}", r); // 2_588_204
    t 
}

pub fn solve() -> u64 {
    original()
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 837799);
    }

    #[bench]
    fn bench_witch_cache(b: &mut test::Bencher) {
        b.iter(|| super::original());
    }

    #[bench]
    fn bench_original(b: &mut test::Bencher) {
        b.iter(|| super::slow());
    }
}
