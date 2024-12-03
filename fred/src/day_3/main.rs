use std::{
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> i64 {
    let file = File::open("src/day_3/input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let regex = Regex::new(r"(?m)mul\([0-9]*,[0-9]*\)").unwrap();
    let mut total = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let c = regex.captures_iter(l.as_str()).collect::<Vec<_>>();
        for capture in c {
            let (a, []) = capture.extract();
            let (lhs, rhs) = a.split_once(",").unwrap();
            let lhs = lhs.replace("mul(", "");
            let rhs = rhs.replace(")", "");
            let lhs = str::parse::<i64>(lhs.as_str()).unwrap();
            let rhs = str::parse::<i64>(rhs.as_str()).unwrap();
            total += rhs * lhs;
        }
    }
    total
}

fn part_2() -> i64 {
    let file = File::open("src/day_3/input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let regex = Regex::new(r"mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut is_recording = true;
    for line in reader.lines() {
        let l = line.unwrap();
        let c = regex.captures_iter(l.as_str()).collect::<Vec<_>>();
        for capture in c {
            let (a, []) = capture.extract();
            match a.split_once(",") {
                Some((lhs, rhs)) => {
                    if is_recording {
                        let lhs = lhs.replace("mul(", "");
                        let rhs = rhs.replace(")", "");
                        let lhs = str::parse::<i64>(lhs.as_str()).unwrap();
                        let rhs = str::parse::<i64>(rhs.as_str()).unwrap();
                        total += rhs * lhs;
                    }
                }
                None => match a {
                    "do()" => is_recording = true,
                    "don't()" => is_recording = false,
                    _ => panic!(),
                },
            }
        }
    }
    total
}
