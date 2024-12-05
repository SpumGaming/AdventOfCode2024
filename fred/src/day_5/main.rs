use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("{}", part_1::do_work());
    println!("{}", part_2::do_work());
}

mod part_1 {
    use std::collections::HashMap;

    use super::*;
    pub fn do_work() -> u32 {
        let file = File::open("src/day_5/input.txt").unwrap();
        let reader = io::BufReader::new(file);
        let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

        let divider_line_index = lines.iter().position(|line| line == "").unwrap();
        let rules = lines[0..divider_line_index]
            .iter()
            .map(|line| line.split_once("|").unwrap())
            .map(|tup| {
                (
                    str::parse::<u32>(tup.0).unwrap(),
                    str::parse::<u32>(tup.1).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let updates = lines[divider_line_index + 1..]
            .iter()
            .map(|line| {
                line.split(",")
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|elem| str::parse::<u32>(elem).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rules_map = {
            let mut rules_map = HashMap::<u32, Vec<u32>>::new();
            for r in rules {
                let (before, after) = r;
                rules_map
                    .entry(before)
                    .and_modify(|must_come_before| must_come_before.push(after))
                    .or_insert(vec![after]);
            }
            rules_map
        };

        let mut correctly_ordered_updates: Vec<Vec<u32>> = vec![];
        for update in &updates {
            if is_update_valid(&rules_map, update) {
                correctly_ordered_updates.push(update.to_owned());
            }
        }

        let total: u32 = correctly_ordered_updates
            .iter()
            .map(|line| line[line.len() / 2])
            .sum();
        total
    }

    fn is_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
        let mut is_valid = true;
        for (index, number) in update.iter().enumerate() {
            let rest = &update[index + 1..];
            for x in rest {
                let fetched = rules.get(x);
                if fetched.is_some() {
                    let f = fetched.unwrap();
                    if f.contains(number) {
                        is_valid = false;
                    }
                }
            }
        }

        is_valid
    }
}

mod part_2 {
    use std::{cmp::Ordering, collections::HashMap};

    use super::*;
    pub fn do_work() -> u32 {
        let file = File::open("src/day_5/input.txt").unwrap();
        let reader = io::BufReader::new(file);
        let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

        let divider_line_index = lines.iter().position(|line| line == "").unwrap();
        let rules = lines[0..divider_line_index]
            .iter()
            .map(|line| line.split_once("|").unwrap())
            .map(|tup| {
                (
                    str::parse::<u32>(tup.0).unwrap(),
                    str::parse::<u32>(tup.1).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let updates = lines[divider_line_index + 1..]
            .iter()
            .map(|line| {
                line.split(",")
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|elem| str::parse::<u32>(elem).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rules_map = {
            let mut rules_map = HashMap::<u32, Vec<u32>>::new();
            for r in rules {
                let (before, after) = r;
                rules_map
                    .entry(before)
                    .and_modify(|must_come_before| must_come_before.push(after))
                    .or_insert(vec![after]);
            }
            rules_map
        };

        let mut incorrectly_ordered_updates: Vec<Vec<u32>> = vec![];
        for update in &updates {
            if !is_update_valid(&rules_map, update) {
                incorrectly_ordered_updates.push(update.to_owned());
            }
        }

        let mut corrected_orders = vec![];
        for update in incorrectly_ordered_updates {
            corrected_orders.push(fix_update(&rules_map, update));
        }

        let total: u32 = corrected_orders
            .iter()
            .map(|line| line[line.len() / 2])
            .sum();
        total
    }

    fn is_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
        let mut is_valid = true;
        for (index, number) in update.iter().enumerate() {
            let rest = &update[index + 1..];
            for x in rest {
                let fetched = rules.get(x);
                if fetched.is_some() {
                    let f = fetched.unwrap();
                    if f.contains(number) {
                        is_valid = false;
                    }
                }
            }
        }

        is_valid
    }

    fn fix_update(rules: &HashMap<u32, Vec<u32>>, mut update: Vec<u32>) -> Vec<u32> {
        update.sort_by(|a, b| {
            if rules.get(a).is_some_and(|pages| pages.contains(b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        update
    }
}
