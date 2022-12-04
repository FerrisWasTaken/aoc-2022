pub fn part1(input: &str) -> u32 {
    let lines = input.split('\n');
    let mut sum = 0u32;
    for line in lines {
        let (sac1, sac2) = divide(line);
        let commons = intersect(sac1, sac2);
        sum += priority(commons);
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut lines = input.split('\n');
    let mut sum = 0u32;
    let mut backpack: Vec<&str> = vec![];
    for i in lines {
        backpack.push(i);
        if backpack.len() == 3 {
            let first = backpack[0];
            let second = backpack[1];
            // dbg!(backpack.clone());
            let shortlist = intersect(first, second);
            let shortlist_c = shortlist;
            let mut shortlist = String::new();
            for j in shortlist_c {
                shortlist.push(j)
            }
            let badge = intersect(shortlist.as_str(), backpack[2]);
            // dbg!(badge.clone());
            sum += priority(badge);
            backpack.clear();
        }
    }
    sum
}

#[inline(always)]
fn divide(input: &str) -> (&str, &str) {
    let len = input.chars().count();
    (&input[..len / 2], &input[len / 2..])
}

fn intersect(sac1: &str, sac2: &str) -> Vec<char> {
    let mut commons = vec![];
    for c in sac1.chars() {
        if sac2.contains(c) && !commons.contains(&c) {
            commons.push(c);
        }
    }
    commons
}

fn priority(input: Vec<char>) -> u32 {
    let mut sum = 0u32;
    for c in input {
        let ascii = c as u8;
        let priority;
        if c.is_ascii_lowercase() {
            priority = ascii - 96;
        } else {
            priority = ascii - 38;
        }
        sum += priority as u32;
    }
    sum
}
