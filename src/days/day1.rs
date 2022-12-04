type Parsed = Vec<u64>;
type Output = u64;

fn parse(input: &str) -> Parsed {
    let mut elves: Vec<u64> = vec![];
    let mut reader = input.split('\n').into_iter();
    elves.push(0);
    while let Some(line) = reader.next() {
        if line == "" {
            elves.push(0);
            continue;
        }
        let line = line.trim();
        let last_i = elves.clone().len() - 1;
        let last = elves.get_mut(last_i).unwrap();
        *last += line.parse::<u64>().expect(format!("{}", line).as_str())
    }
    elves
}

pub fn part1(data: &str) -> Output {
    let data = parse(data);
    *data.iter().max().unwrap()
}

pub fn part2(data: &str) -> Output {
    let data = parse(data);
    let mut buf = data.clone();
    let mut top = vec![];
    for _ in 0..3 {
        let mut largest = (0, 0u64);
        for (i, x) in buf.iter().enumerate() {
            if x > &largest.1 {
                largest.1 = *x;
                largest.0 = i;
            }
        }
        top.push(largest.1);
        buf.remove(largest.0);
    }
    top.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        assert_eq!(part1(include_str!("../../input/2022/day1.txt")), 69912);
    }

    #[test]
    fn d1p2() {
        assert_eq!(part2(include_str!("../../input/2022/day1.txt")), 208180);
    }
}