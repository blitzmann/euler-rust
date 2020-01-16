const number: &str = "\
73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450";

pub fn solve(window: usize) -> u64 {
    original(window)
}

pub fn original(n: usize) -> u64 {
    number
        .chars() // convert string to characters
        .map(|s| s.to_digit(10).unwrap() as u64) // map to digits
        .collect::<Vec<u64>>() // collect into a Vector so that we can...
        .windows(n) // ... use windows function to get an iterator of all the various windows
        .map(|w| w.into_iter().product::<u64>()) // which we then multiply together
        .fold(0u64, |a, b| a.max(b)) // and fold by always taking the max one in the accumulator
}

pub fn iteration(n: usize) -> u64 {
    let input = number;

    let mut largest = 0;
    let input_bytes = input.as_bytes();
    let mut largest_string: &[u8] = &input_bytes[0..1];

    let span_width = n;

    for i in 0..(input_bytes.len() - span_width + 1) {
        let mut sum = 1u64;
        for j in 0..(span_width) {
            sum *= (input_bytes[i + j] - 48) as u64;
        }
        if sum > largest {
            largest = sum;
            largest_string = &input_bytes[i..(i + span_width)];
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(4), 5832);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(13), 23514624000);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| super::solve(13));
    }

    #[bench]
    fn bench_iteration(b: &mut test::Bencher) {
        b.iter(|| super::iteration(13));
    }
}
