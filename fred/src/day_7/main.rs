use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("{}", part_1::do_work());
    println!("{}", part_2::do_work());
}

mod part_1 {
    use itertools::Itertools;
    

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum EquationComponent {
        Number(u64),
        Operator(Operator),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Operator {
        Plus,
        Multiply,
    }

    pub fn do_work() -> u64 {
        let file = File::open("src/day_7/input.txt").unwrap();
        let reader = io::BufReader::new(file);

        let problems = reader
            .lines()
            .map(|line| {
                let binding = line.unwrap();
                let (target, rest) = binding.split_once(":").unwrap();
                let numbers = rest[1..]
                    .split(" ")
                    .map(str::parse::<u64>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>();
                (str::parse::<u64>(target).unwrap(), numbers)
            })
            .collect::<Vec<_>>();

        let total: u64 = problems
            .into_par_iter()
            .enumerate()
            .filter(|(line, p)| {
                let (target, nums) = p;
                is_solvable(*target, nums)
            })
            .map(|p| p.1 .0)
            .sum();

        total
    }

    fn is_solvable(target: u64, nums: &Vec<u64>) -> bool {
        if nums.len() == 1 {
            return target == nums[0];
        }
        let operator_count = nums.len() - 1;

        let permutations = (0..operator_count)
            .map(|_| [Operator::Multiply, Operator::Plus])
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        permutations.iter().any(|permutation| {
            let mut operator_iter = permutation.iter();
            target
                == nums
                    .iter()
                    .copied()
                    .reduce(|lhs, rhs| match operator_iter.next().unwrap() {
                        Operator::Plus => lhs + rhs,
                        Operator::Multiply => lhs * rhs,
                    })
                    .unwrap()
        })
    }
}

mod part_2 {
    use itertools::Itertools;
    

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum EquationComponent {
        Number(u64),
        Operator(Operator),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Operator {
        Plus,
        Multiply,
        Concatenate,
    }

    pub fn do_work() -> u64 {
        let file = File::open("src/day_7/input.txt").unwrap();
        let reader = io::BufReader::new(file);

        let problems = reader
            .lines()
            .map(|line| {
                let binding = line.unwrap();
                let (target, rest) = binding.split_once(":").unwrap();
                let numbers = rest[1..]
                    .split(" ")
                    .map(str::parse::<u64>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>();
                (str::parse::<u64>(target).unwrap(), numbers)
            })
            .collect::<Vec<_>>();

        let total: u64 = problems
            .into_par_iter()
            .enumerate()
            .filter(|(line, p)| {
                let (target, nums) = p;
                is_solvable(*target, nums)
            })
            .map(|p| p.1 .0)
            .sum();

        total
    }

    fn is_solvable(target: u64, nums: &Vec<u64>) -> bool {
        if nums.len() == 1 {
            return target == nums[0];
        }
        let operator_count = nums.len() - 1;

        let permutations = (0..operator_count)
            .map(|_| [Operator::Multiply, Operator::Plus, Operator::Concatenate])
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        permutations.iter().any(|permutation| {
            let mut operator_iter = permutation.iter();
            target
                == nums
                    .iter()
                    .copied()
                    .reduce(|lhs, rhs| match operator_iter.next().unwrap() {
                        Operator::Plus => lhs + rhs,
                        Operator::Multiply => lhs * rhs,
                        Operator::Concatenate => {
                            str::parse::<u64>(&format!("{}{}", lhs, rhs))
                                .unwrap()
                        }
                    })
                    .unwrap()
        })
    }
}
