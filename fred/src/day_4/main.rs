use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("{}", part_1::part_1());
    println!("{}", part_2::part_2());
}

mod part_1 {
    use super::*;
    pub fn part_1() -> i32 {
        let file = File::open("src/day_4/input.txt").unwrap();
        let reader = io::BufReader::new(file);

        let lines = reader
            .lines()
            .map(Result::unwrap)
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut matches = 0;
        for (line_index, line) in lines.iter().enumerate() {
            for (column_index, char) in line.iter().enumerate() {
                if *char == 'X' {
                    if column_index >= 3 {
                        if line[column_index - 1] == 'M'
                            && line[column_index - 2] == 'A'
                            && line[column_index - 3] == 'S'
                        {
                            matches += 1
                        }
                        if line_index >= 3 {
                            if lines[line_index - 1][column_index - 1] == 'M'
                                && lines[line_index - 2][column_index - 2] == 'A'
                                && lines[line_index - 3][column_index - 3] == 'S'
                            {
                                matches += 1
                            }
                        }
                        if line_index + 3 < lines.len() {
                            if lines[line_index + 1][column_index - 1] == 'M'
                                && lines[line_index + 2][column_index - 2] == 'A'
                                && lines[line_index + 3][column_index - 3] == 'S'
                            {
                                matches += 1
                            }
                        }
                    }
                    if column_index + 3 < line.len() {
                        if line[column_index + 1] == 'M'
                            && line[column_index + 2] == 'A'
                            && line[column_index + 3] == 'S'
                        {
                            matches += 1
                        }
                        if line_index >= 3 {
                            if lines[line_index - 1][column_index + 1] == 'M'
                                && lines[line_index - 2][column_index + 2] == 'A'
                                && lines[line_index - 3][column_index + 3] == 'S'
                            {
                                matches += 1
                            }
                        }
                        if line_index + 3 < lines.len() {
                            if lines[line_index + 1][column_index + 1] == 'M'
                                && lines[line_index + 2][column_index + 2] == 'A'
                                && lines[line_index + 3][column_index + 3] == 'S'
                            {
                                matches += 1
                            }
                        }
                    }
                    if line_index >= 3 {
                        if lines[line_index - 1][column_index] == 'M'
                            && lines[line_index - 2][column_index] == 'A'
                            && lines[line_index - 3][column_index] == 'S'
                        {
                            matches += 1
                        }
                    }
                    if line_index + 3 < lines.len() {
                        if lines[line_index + 1][column_index] == 'M'
                            && lines[line_index + 2][column_index] == 'A'
                            && lines[line_index + 3][column_index] == 'S'
                        {
                            matches += 1
                        }
                    }
                }
            }
        }
        matches
    }
}

mod part_2 {
    use super::*;
    pub fn part_2() -> i32 {
        let file = File::open("src/day_4/input.txt").unwrap();
        let reader = io::BufReader::new(file);

        let lines = reader
            .lines()
            .map(Result::unwrap)
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut matches = 0;
        for (line_index, line) in lines.iter().enumerate() {
            for (column_index, char) in line.iter().enumerate() {
                if *char == 'A'
                    && column_index > 0
                    && column_index + 1 < line.len()
                    && line_index > 0
                    && line_index + 1 < lines.len()
                {
                    let chars: Vec<char> = vec![
                        lines[line_index - 1][column_index - 1],
                        lines[line_index - 1][column_index + 1],
                        lines[line_index + 1][column_index + 1],
                        lines[line_index + 1][column_index - 1],
                    ];
                    if chars == ['M', 'M', 'S', 'S']
                        || chars == ['M', 'S', 'S', 'M']
                        || chars == ['S', 'S', 'M', 'M']
                        || chars == ['S', 'M', 'M', 'S']
                    {
                        matches += 1
                    }
                }
            }
        }
        matches
    }
}
