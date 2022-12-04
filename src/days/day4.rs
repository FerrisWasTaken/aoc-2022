pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| occurence_map(line)).sum()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}

#[derive(Debug, PartialEq)]
struct Elf {
    start: i32,
    end: i32,
}

fn occurence_map(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(",").collect();
    let part1: Vec<u64> = parts[0]
        .split("-")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let part2: Vec<u64> = parts[1]
        .split("-")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let (a, b) = (part1[0], part1[1]);
    let (x, y) = (part2[0], part2[1]);
    // Thx to @dsouzadyn on github for this snippet
    // condition if a and b are smaller than equal to x and y
    let is_valid = (a >= x && b <= y) || (x >= a && y <= b);
    match is_valid {
        true => 1,
        false => 0,
    }
}