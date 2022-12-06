use regex::Regex;

pub fn part1(input: &str) -> String {
    let mut ans = String::new();
    let mut crates: Vec<Vec<char>> = vec![];
    let mut lines = input.split('\n');
    for i in &mut lines {
        if i == "" {
            break;
        }
        for j in (1..i.len()).step_by(4) {
            let c = i.chars().nth(j).unwrap();
            if c.is_numeric() {
                break;
            }

            let pos = j / 4;
            if crates.get(pos).is_none() {
                crates.push(vec![]);
            }

            if c.is_alphabetic() {
                crates[pos].push(c);
            }
        }
    }
    for i in crates.iter_mut() {
        i.reverse()
    }
    dbg!(crates.clone());
    for instr in lines {
        if instr == "" {
            continue;
        }
        mv(&mut crates, instr)
    }
    for col in crates {
        ans.push(*col.last().unwrap())
    }
    ans
}

pub fn part2(input: &str) -> String {
    let mut ans = String::new();
    let mut crates: Vec<Vec<char>> = vec![];
    let mut lines = input.split('\n');
    for i in &mut lines {
        if i == "" {
            break;
        }
        for j in (1..i.len()).step_by(4) {
            let c = i.chars().nth(j).unwrap();
            if c.is_numeric() {
                break;
            }

            let pos = j / 4;
            if crates.get(pos).is_none() {
                crates.push(vec![]);
            }

            if c.is_alphabetic() {
                crates[pos].push(c);
            }
        }
    }
    for i in crates.iter_mut() {
        i.reverse()
    }
    dbg!(crates.clone());
    for instr in lines {
        if instr == "" {
            continue;
        }
        mv_9001(&mut crates, instr)
    }
    for col in crates {
        ans.push(*col.last().unwrap())
    }
    ans
}

fn mv(crates: &mut Vec<Vec<char>>, instr: &str) {
    let (count, src, dest) = parse_inst(instr);
    let mut buf: Vec<char> = vec![];
    let src = crates.get_mut(src-1).unwrap();
    for _ in 0..count {
        buf.push(src.pop().unwrap());
    }
    drop(src);
    let dest = crates.get_mut(dest-1).unwrap();
    for i in buf {
        dest.push(i)
    }
}

fn mv_9001(crates: &mut Vec<Vec<char>>, instr: &str) {
    let (count, src, dest) = parse_inst(instr);
    let mut buf: Vec<char> = vec![];
    let src = crates.get_mut(src-1).unwrap();
    for _ in 0..count {
        buf.insert(0, src.pop().unwrap());
    }
    drop(src);
    let dest = crates.get_mut(dest-1).unwrap();
    for i in buf {
        dest.push(i)
    }
}

fn parse_inst(instr: &str) -> (usize, usize, usize) {
    let regex = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
    let mut iter =  regex.captures(instr).unwrap();
    (iter[1].parse::<usize>().unwrap(), iter[2].parse::<usize>().unwrap(), iter[3].parse::<usize>().unwrap())
}