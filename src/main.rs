use std::{fs, fmt::Display, env::current_exe};

macro_rules! run {
    ($d:expr, $p1:expr, $p2:expr) => {
        println!("Day {}", $d);
        if $p1 {
            paste::expr! {
                run_part(days::[<day $d>]::part1, $d, 1)
            }
        }
        if $p2 {
            paste::expr! {
                run_part(days::[<day $d>]::part2, $d, 2)
            }
        }
    };
}

const YEAR: u32 = 2022;

fn main() {
    run!(1, true, true);
    run!(2, true, true);
    run!(3, true, true);
}

fn run_part<T: Display>(func: impl Fn(&str) -> T, day: u32, part: u32) {
    let mut buf = current_exe().unwrap();
    buf.pop();
    buf.pop();
    buf.pop();
    buf.push("input");
    buf.push(format!("{}", YEAR));
    buf.push(format!("day{}.txt", day));
    let file = fs::read_to_string(buf).unwrap();
    println!("  Part {}: {}", part, func(file.as_str()));
}
