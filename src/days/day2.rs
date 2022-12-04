type Parsed = Vec<(char, char)>;
type Output = u32;

fn parse(input: &str) -> Parsed {
    let lines = input.split('\n');
    let mut plays: Vec<(char, char)> = vec![];
    for line in lines {
        let line = line.trim();
        let mut letters = line.split(' ');
        let l1 = letters.next().unwrap();
        let l2 = letters.next().unwrap();
        drop(letters);
        plays.push((l1.chars().next().unwrap(), l2.chars().next().unwrap()));
    }
    plays
}

pub fn part1(data: &str) -> Output {
    let data = parse(data);
    let mut score = 0u32;
    for (opponent, you) in data.iter() {
        match *you {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => unreachable!(),
        }
        match *opponent {
            'A' if *you == 'X' => score += 3,
            'B' if *you == 'Y' => score += 3,
            'C' if *you == 'Z' => score += 3,
            _ => {}
        }
        match *opponent {
            'A' if *you == 'Y' => score += 6,
            'B' if *you == 'Z' => score += 6,
            'C' if *you == 'X' => score += 6,
            _ => continue,
        }
    }
    score
}

pub fn part2(data: &str) -> Output {
    let data = parse(data);
    let mut score = 0;
    for (opponent, you) in data {
        match you {
            'X' => match opponent {
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => unreachable!(),
            },
            'Y' => {
                match opponent {
                    'A' => score += 1,
                    'B' => score += 2,
                    'C' => score += 3,
                    _ => unreachable!(),
                }
                score += 3
            }
            'Z' => {
                match opponent {
                    'A' => score += 2,
                    'B' => score += 3,
                    'C' => score += 1,
                    _ => unreachable!(),
                }
                score += 6
            }
            _ => unreachable!(),
        }
    }
    score
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn d2p1() {
    assert_eq!(part1(include_str!("../../input/2022/day2.txt")), 10404);
}

#[test]
fn d2p2() {
    assert_eq!(part2(include_str!("../../input/2022/day2.txt")), 10334);
}
}
