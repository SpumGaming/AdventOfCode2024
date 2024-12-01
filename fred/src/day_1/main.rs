use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> usize {
    let file = File::open("src/day_1/part_1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let lists = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split_whitespace();
            (
                str::parse::<usize>(split.next().unwrap()).unwrap(),
                str::parse::<usize>(split.next().unwrap()).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut left_list = lists.iter().map(|line| line.0).collect::<Vec<_>>();
    let mut right_list = lists.iter().map(|line| line.1).collect::<Vec<_>>();
    left_list.sort();
    right_list.sort();

    let distances = left_list
        .iter()
        .zip(right_list.iter())
        .map(|tuple| ((*tuple.0 as i128) - (*tuple.1 as i128)).abs())
        .collect::<Vec<_>>();

    let total_distance = distances
        .into_iter()
        .reduce(|acc, distance| acc + distance)
        .unwrap();

    total_distance as usize
}

fn part_2() -> usize {
    let file = File::open("src/day_1/part_1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let lists = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split_whitespace();
            (
                str::parse::<usize>(split.next().unwrap()).unwrap(),
                str::parse::<usize>(split.next().unwrap()).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut left_list = lists.iter().map(|line| line.0).collect::<Vec<_>>();
    let mut right_list = lists.iter().map(|line| line.1).collect::<Vec<_>>();
    left_list.sort();
    right_list.sort();

    let similarity_score = left_list
        .iter()
        .map(|elem| (*elem * right_list.iter().filter(|num| **num == *elem).count()))
        .reduce(|acc, e| acc + e)
        .unwrap();

    similarity_score
}
