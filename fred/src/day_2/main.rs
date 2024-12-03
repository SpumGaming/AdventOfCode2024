use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> i32 {
    let file = File::open("src/day_2/part_1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;
    for level in reader.lines() {
        let reports = level.unwrap();
        let reports = reports
            .split_whitespace()
            .map(|val| str::parse::<i32>(val).unwrap())
            .collect::<Vec<_>>();

        let is_all_increasing = is_gradually_increasing_array(&reports);
        let is_all_decreasing = is_gradually_decreasing_array(&reports);
        if is_all_decreasing || is_all_increasing {
            safe_count += 1;
        }
    }

    safe_count
}
fn part_2() -> i32 {
    let file = File::open("src/day_2/part_1.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;
    for level in reader.lines() {
        let reports = level.unwrap();
        let reports = reports
            .split_whitespace()
            .map(|val| str::parse::<i32>(val).unwrap())
            .collect::<Vec<_>>();

        let is_all_increasing = part_2::is_gradually_increasing_array(reports.clone());
        let is_all_decreasing = part_2::is_gradually_decreasing_array(reports.clone());
        if is_all_decreasing || is_all_increasing {
            safe_count += 1;
        }
    }

    safe_count
}

mod part_2 {
    pub(crate) fn is_gradually_increasing_array(input: Vec<i32>) -> bool {
        if input
            .windows(2)
            .all(|adjacent_vals| is_gradually_increasing((adjacent_vals[0], adjacent_vals[1])))
        {
            return true;
        } else {
            for index in 0..input.len() {
                let mut input_clone = input.clone();
                input_clone.remove(index);
                if input_clone.windows(2).all(|adjacent_vals| {
                    is_gradually_increasing((adjacent_vals[0], adjacent_vals[1]))
                }) {
                    return true;
                }
            }
        }

        false
    }

    pub(crate) fn is_gradually_decreasing_array(input: Vec<i32>) -> bool {
        if input
            .windows(2)
            .all(|adjacent_vals| is_gradually_decreasing((adjacent_vals[0], adjacent_vals[1])))
        {
            return true;
        } else {
            for index in 0..input.len() {
                let mut input_clone = input.clone();
                input_clone.remove(index);
                if input_clone.windows(2).all(|adjacent_vals| {
                    is_gradually_decreasing((adjacent_vals[0], adjacent_vals[1]))
                }) {
                    return true;
                }
            }
        }

        false
    }

    fn is_gradually_decreasing(input: (i32, i32)) -> bool {
        let sum = input.0 - input.1;
        (1..=3).contains(&sum)
    }

    fn is_gradually_increasing(input: (i32, i32)) -> bool {
        let diff = input.1 - input.0;
        (1..=3).contains(&diff)
    }
}

fn is_gradually_increasing_array(input: &[i32]) -> bool {
    let mut window = input.windows(2);

    window.all(|adjacent_vals| is_gradually_increasing((adjacent_vals[0], adjacent_vals[1])))
}

fn is_gradually_decreasing_array(input: &[i32]) -> bool {
    let mut window = input.windows(2);

    window.all(|adjacent_vals| is_gradually_decreasing((adjacent_vals[0], adjacent_vals[1])))
}

fn is_gradually_decreasing(input: (i32, i32)) -> bool {
    let sum = input.0 - input.1;
    (1..=3).contains(&sum)
}

fn is_gradually_increasing(input: (i32, i32)) -> bool {
    let diff = input.1 - input.0;
    (1..=3).contains(&diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = (3, 0);
        assert!(is_gradually_decreasing(input));

        let input = (3, 3);
        assert!(!is_gradually_decreasing(input));

        let input = (0, 3);
        assert!(!is_gradually_decreasing(input));
    }

    #[test]
    fn test_2() {
        let input = (0, 3);
        assert!(is_gradually_increasing(input));

        let input = (3, 0);
        assert!(!is_gradually_increasing(input));

        let input = (0, 1);
        assert!(is_gradually_increasing(input));

        let input = (0, 0);
        assert!(!is_gradually_increasing(input));
    }
    #[test]
    fn test_3() {
        let input = [7, 6, 4, 2, 1];
        assert!(!is_gradually_increasing_array(&input));

        let input = [1, 2, 7, 8, 9];
        assert!(!is_gradually_increasing_array(&input));

        let input = [9, 7, 6, 2, 1];
        assert!(!is_gradually_increasing_array(&input));

        let input = [1, 3, 2, 4, 5];
        assert!(!is_gradually_increasing_array(&input));

        let input = [8, 6, 4, 4, 1];
        assert!(!is_gradually_increasing_array(&input));

        let input = [1, 3, 6, 7, 9];
        assert!(is_gradually_increasing_array(&input));
    }

    #[test]
    fn test_4() {
        let input = [7, 6, 4, 2, 1];
        assert!(is_gradually_decreasing_array(&input));

        let input = [1, 2, 7, 8, 9];
        assert!(!is_gradually_decreasing_array(&input));

        let input = [9, 7, 6, 2, 1];
        assert!(!is_gradually_decreasing_array(&input));

        let input = [1, 3, 2, 4, 5];
        assert!(!is_gradually_decreasing_array(&input));

        let input = [8, 6, 4, 4, 1];
        assert!(!is_gradually_decreasing_array(&input));

        let input = [1, 3, 6, 7, 9];
        assert!(!is_gradually_decreasing_array(&input));
    }

    #[test]
    fn test_5() {
        let input = vec![7, 6, 4, 2, 1];
        assert!(part_2::is_gradually_decreasing_array(input));

        let input = vec![1, 2, 7, 8, 9];
        assert!(!part_2::is_gradually_decreasing_array(input));

        let input = vec![9, 7, 6, 2, 1];
        assert!(!part_2::is_gradually_decreasing_array(input));

        let input = vec![1, 3, 2, 4, 5];
        assert!(!part_2::is_gradually_decreasing_array(input));

        let input = vec![8, 6, 4, 4, 1];
        assert!(part_2::is_gradually_decreasing_array(input));

        let input = vec![1, 3, 6, 7, 9];
        assert!(!part_2::is_gradually_decreasing_array(input));

        let input = vec![3, 2, 3, 4, 5];
        assert!(part_2::is_gradually_increasing_array(input));

        let input = vec![3, 3, 3, 4, 5];
        assert!(!part_2::is_gradually_increasing_array(input));

        let input = vec![3, 4, 3, 5];
        assert!(part_2::is_gradually_increasing_array(input));
    }

    #[test]
    fn test_6() {
        let input = vec![7, 6, 4, 2, 1];
        assert!(!part_2::is_gradually_increasing_array(input));

        let input = vec![1, 2, 7, 8, 9];
        assert!(!part_2::is_gradually_increasing_array(input));

        let input = vec![9, 7, 6, 2, 1];
        assert!(!part_2::is_gradually_increasing_array(input));

        let input = vec![1, 3, 2, 4, 5];
        assert!(part_2::is_gradually_increasing_array(input));

        let input = vec![8, 6, 4, 4, 1];
        assert!(!part_2::is_gradually_increasing_array(input));

        let input = vec![1, 3, 6, 7, 9];
        assert!(part_2::is_gradually_increasing_array(input));

        let input = vec![1, 3, 2, 3, 4];
        assert!(part_2::is_gradually_increasing_array(input));

        let input = vec![62, 65, 67, 70, 73, 76, 75];
        assert!(part_2::is_gradually_increasing_array(input));
    }
}

// between 424 and 447
