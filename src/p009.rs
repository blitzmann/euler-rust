pub fn solve() -> u32 {
    iteration()
}

pub fn iteration() -> u32 {
    let mut amin: u32 = 1;
    let mut bmin: u32 = 2;
    let mut cmin: u32 = 3;
    'aloop: for a in amin..1000 {
        'bloop: for b in a..1000 {
            'cloop: for c in b..1000 {
                if a + b + c == 1000 && (a.pow(2) + b.pow(2) == c.pow(2)) {
                    return a * b * c;
                }
            }
        }
    }
    0
}

// Brute force method
pub fn original_brute_force() -> u32 {
    let mut amin: u32 = 1;
    let mut bmin: u32 = 2;
    let mut cmin: u32 = 3;
    'aloop: for a in amin..1000 {
        'bloop: for b in bmin..1000 {
            'cloop: for c in cmin..1000 {
                if a + b + c == 1000 && (a.pow(2) + b.pow(2) == c.pow(2)) {
                    return a * b * c;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 31875000);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| super::original_brute_force());
    }

    #[bench]
    fn bench_brute_force(b: &mut test::Bencher) {
        b.iter(|| super::iteration());
    }
}
