/*
    For this, we will simply be walking the list. As we walk the list, we will check
    the four adjacent numbers going down, right, down-left, and down-right. We use
    these directions to simplify the logic:
    * We don't have to check for negative indexes, only out of bounds
    * We don't need to check the other four directions as they will be checked with another index test
*/

const WIDTH: u32 = 20;
const ARM_LENGTH: u32 = 4;
const GRID: &str = "\
    08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
    49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
    81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
    52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
    22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
    24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
    32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
    67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
    24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
    21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
    78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
    16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
    86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
    19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
    04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
    88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
    04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
    20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
    20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
    01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

fn idx_vert(i: usize, list: &Vec<u32>) -> u32 {
    if (i as u32 > list.len() as u32 - (WIDTH * (ARM_LENGTH - 1))) {
        return 0;
    }
    let values: Option<Vec<_>> = (0..ARM_LENGTH)
        .map(|x| i as u32 + (WIDTH * x))
        .map(|t| list.get(t as usize))
        .collect(); // collect into a single Option which may have a vec

    match values {
        None => 0, // happens when out of bounds, so it's effectively 0
        Some(vec) => vec.into_iter().product(),
    }
}

fn idx_hor(i: usize, list: &Vec<u32>) -> u32 {
    // if our initial one is too close to the right edge, return early
    if (WIDTH - (i as u32 % WIDTH) < ARM_LENGTH) {
        return 0;
    }
    let values: Option<Vec<_>> = (0..ARM_LENGTH)
        .map(|x| i as u32 + x)
        .map(|t| list.get(t as usize))
        .collect(); // collect into a single Option which may have a vec

    match values {
        None => 0, // happens when out of bounds, so it's effectively 0
        Some(vec) => vec.into_iter().product(),
    }
}

fn idx_diag_left(i: usize, list: &Vec<u32>) -> u32 {
    if (
        i as u32 > list.len() as u32 - (WIDTH * (ARM_LENGTH-1)) // too far down, same as the vert check
        || i as u32 % WIDTH < ARM_LENGTH-1
        // too far to the left
    ) {
        return 0;
    }
    let values: Option<Vec<_>> = (0..ARM_LENGTH)
        .map(|x| i as u32 + (WIDTH * x) - x)
        .map(|t| list.get(t as usize))
        .collect();

    match values {
        None => 0, // happens when out of bounds, so it's effectively 0
        Some(vec) => vec.into_iter().product(),
    }
}

fn idx_diag_right(i: usize, list: &Vec<u32>) -> u32 {
    if (
        i as u32 > list.len() as u32 - (WIDTH * (ARM_LENGTH-1)) // too far down, same as the vert check
        || WIDTH  - (i as u32 % WIDTH) < ARM_LENGTH
        // too far to the right
    ) {
        return 0;
    }
    let values: Option<Vec<_>> = (0..ARM_LENGTH)
        .map(|x| i as u32 + (WIDTH * x) + x)
        // .inspect(|x| println!("t: {:?}", x))
        .map(|t| list.get(t as usize))
        // .inspect(|x| println!("r: {:?}", x))
        .collect(); // collect into a single Option which may have a vec

    match values {
        None => 0, // happens when out of bounds, so it's effectively 0
        Some(vec) => vec.into_iter().product(),
    }
}

pub fn solve() -> u32 {
    let funcs: Vec<fn(usize, &Vec<u32>) -> u32> =
        vec![idx_diag_left, idx_diag_right, idx_hor, idx_vert];

    let list: Vec<u32> = GRID
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    // this doesn't work like it should :(
    //     let r = &list
    //         .into_iter()
    //         .enumerate()
    //         .map(|(i, n)| &funcs
    //                         .into_iter()
    //                         .map(|func| func(i, &list))
    //                         .fold(0, |a, b| a.max(b))
    //         )
    //         .fold(0, |a, b| a.max(b));

    let mut maxmax = 0;
    for (i, x) in list.iter().enumerate() {
        let max = &funcs
            .iter()
            .map(|func| func(i, &list))
            .fold(0, |a, b| a.max(b));

        if max > &maxmax {
            maxmax = *max;
        }
    }
    maxmax
}

#[cfg(test)]
mod tests {

    #[test]
    fn idx_vert() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_vert(2, &list), 22 * 99 * 31 * 95);
    }

    #[test]
    fn idx_vert_out_of_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_vert(360, &list), 0);
    }

    #[test]
    fn idx_vert_last_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_vert(339, &list), 36 * 16 * 54 * 48);
    }

    #[test]
    fn idx_hor() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_hor(1, &list), 2 * 22 * 97 * 38);
    }

    #[test]
    fn idx_hor_out_of_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_hor(399, &list), 0); // Out of bounds
    }

    #[test]
    fn idx_hor_left_bound() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_hor(20, &list), 49 * 49 * 99 * 40);
    }

    #[test]
    fn idx_hor_right_bound() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_hor(16, &list), 50 * 77 * 91 * 08);
    }
    #[test]
    fn idx_hor_wrapped() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_hor(17, &list), 0); // wrapped to next line
    }

    #[test]
    fn idx_diag_left() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_left(3, &list), 97 * 99 * 49 * 52);
    }

    #[test]
    fn idx_diag_left_wrapped() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_left(22, &list), 0);
    }

    #[test]
    fn idx_diag_left_out_of_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_left(361, &list), 0);
    }

    #[test]
    fn idx_diag_left_left_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_left(323, &list), 73 * 36 * 73 * 1);
    }

    #[test]
    fn idx_diag_left_right_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_left(339, &list), 36 * 36 * 57 * 89);
    }

    #[test]
    fn idx_diag_right() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_right(0, &list), 8 * 49 * 31 * 23); // out of bounds
    }

    #[test]
    fn idx_diag_right_left_bound() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_right(80, &list), 22 * 47 * 81 * 68); // out of bounds
    }

    #[test]
    fn idx_diag_right_right_bound() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_right(16, &list), 50 * 56 * 36 * 91); // out of bounds
    }

    #[test]
    fn idx_diag_right_out_of_bounds() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_right(340, &list), 0); // out of bounds
    }

    #[test]
    fn idx_diag_right_wrapped() {
        let list: Vec<u32> = super::GRID
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();
        assert_eq!(super::idx_diag_right(17, &list), 0); // out of bounds
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 70600674);
    }
}
