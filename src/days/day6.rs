use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut count = 0usize;
    let mut iter = input.chars();
    while let Some(c1) = iter.next() {
        let mut iter = iter.clone();
        let c2 = iter.next().unwrap();
        let c3 = iter.next().unwrap();
        let c4 = iter.next().unwrap();
        let mut chars = vec![c1, c2, c3, c4];
        chars.sort();
        if chars.iter().all_unique() {
            return count + 4
        } else {
            count += 1
        }
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut count = 0usize;
    let mut iter = input.chars();
    while let Some(c1) = iter.next() {
        let mut iter = iter.clone();
        let c2 = iter.next().unwrap();
        let c3 = iter.next().unwrap();
        let c4 = iter.next().unwrap();
        let c5 = iter.next().unwrap();
        let c6 = iter.next().unwrap();
        let c7 = iter.next().unwrap();
        let c8 = iter.next().unwrap();
        let c9 = iter.next().unwrap();
        let c10 = iter.next().unwrap();
        let c11 = iter.next().unwrap();
        let c12 = iter.next().unwrap();
        let c13 = iter.next().unwrap();
        let c14 = iter.next().unwrap();
        let mut chars = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14];
        chars.sort();
        if chars.iter().all_unique() {
            return count + 14
        } else {
            count += 1
        }
    }
    count
}