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
        if is_all_decreasing == true || is_all_increasing == true {
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
        if is_all_decreasing == true || is_all_increasing == true {
            safe_count += 1;
        }
    }

    safe_count
}

mod part_2 {
    pub(crate) fn is_gradually_increasing_array(mut input: Vec<i32>) -> bool {
        dbg!(&input);
        let mut removal_made = false;
        let mut removal_index;
        let mut index = 0;
        while index != input.len() - 1 {
            let tup = (input[index], input[index + 1]);
            let is_increasing_tuple = is_gradually_increasing(tup);
            if is_increasing_tuple == false {
                if removal_made == true {
                    return false;
                } else {
                    // check if close enough to end of array and if so and the swap isn't made we can just return true
                    if index + 2 >= input.len() {
                        return true;
                    }

                    if is_gradually_decreasing((input[index], input[index + 2])) {
                        removal_index = index + 1
                    } else {
                        removal_index = index;
                    }
                    println!(
                        "removing value: {} at index: {}",
                        input.get(removal_index).unwrap(),
                        removal_index
                    );
                    removal_made = true;
                    input.remove(removal_index);
                    index = 0;
                    continue;
                }
            }
            index += 1;
        }
        true
    }

    pub(crate) fn is_gradually_decreasing_array(mut input: Vec<i32>) -> bool {
        dbg!(&input);
        let mut removal_index;
        let mut removal_made = false;
        let mut index = 0;
        while index != input.len() - 1 {
            let tup = (input[index], input[index + 1]);
            println!("tupl: {:?}", tup);
            let is_decreasing_tuple = self::is_gradually_decreasing(tup);
            println!("is_decreasing?: {}", is_decreasing_tuple);
            if is_decreasing_tuple == false {
                if removal_made == true {
                    return false;
                } else {
                    // check if close enough to end of array and if so and the swap isn't made we can just return true
                    if index + 2 >= input.len() {
                        return true;
                    }

                    if is_gradually_decreasing((input[index], input[index + 2])) {
                        removal_index = index + 1
                    } else {
                        removal_index = index;
                    }
                    println!(
                        "removing value: {} at index: {}",
                        input.get(removal_index).unwrap(),
                        removal_index
                    );
                    removal_made = true;
                    input.remove(removal_index);
                    index = 0;
                    continue;
                }
            }
            index += 1;
        }
        true
    }

    fn is_gradually_decreasing(input: (i32, i32)) -> bool {
        let sum = (input.0 - input.1);
        if sum >= 1 && sum <= 3 {
            true
        } else {
            false
        }
    }

    fn is_gradually_increasing(input: (i32, i32)) -> bool {
        let diff = (input.1 - input.0);
        if diff >= 1 && diff <= 3 {
            true
        } else {
            false
        }
    }
}

fn is_gradually_increasing_array(input: &[i32]) -> bool {
    let mut window = input.windows(2);
    let is_all_increasing =
        window.all(|adjacent_vals| is_gradually_increasing((adjacent_vals[0], adjacent_vals[1])));
    is_all_increasing
}

fn is_gradually_decreasing_array(input: &[i32]) -> bool {
    let mut window = input.windows(2);
    let is_all_increasing =
        window.all(|adjacent_vals| is_gradually_decreasing((adjacent_vals[0], adjacent_vals[1])));
    is_all_increasing
}

fn is_gradually_decreasing(input: (i32, i32)) -> bool {
    let sum = (input.0 - input.1);
    if sum >= 1 && sum <= 3 {
        true
    } else {
        false
    }
}

fn is_gradually_increasing(input: (i32, i32)) -> bool {
    let diff = (input.1 - input.0);
    if diff >= 1 && diff <= 3 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = (3, 0);
        assert_eq!(is_gradually_decreasing(input), true);

        let input = (3, 3);
        assert_eq!(is_gradually_decreasing(input), false);

        let input = (0, 3);
        assert_eq!(is_gradually_decreasing(input), false);
    }

    #[test]
    fn test_2() {
        let input = (0, 3);
        assert_eq!(is_gradually_increasing(input), true);

        let input = (3, 0);
        assert_eq!(is_gradually_increasing(input), false);

        let input = (0, 1);
        assert_eq!(is_gradually_increasing(input), true);

        let input = (0, 0);
        assert_eq!(is_gradually_increasing(input), false);
    }
    #[test]
    fn test_3() {
        let input = [7, 6, 4, 2, 1];
        assert_eq!(is_gradually_increasing_array(&input), false);

        let input = [1, 2, 7, 8, 9];
        assert_eq!(is_gradually_increasing_array(&input), false);

        let input = [9, 7, 6, 2, 1];
        assert_eq!(is_gradually_increasing_array(&input), false);

        let input = [1, 3, 2, 4, 5];
        assert_eq!(is_gradually_increasing_array(&input), false);

        let input = [8, 6, 4, 4, 1];
        assert_eq!(is_gradually_increasing_array(&input), false);

        let input = [1, 3, 6, 7, 9];
        assert_eq!(is_gradually_increasing_array(&input), true);
    }

    #[test]
    fn test_4() {
        let input = [7, 6, 4, 2, 1];
        assert_eq!(is_gradually_decreasing_array(&input), true);

        let input = [1, 2, 7, 8, 9];
        assert_eq!(is_gradually_decreasing_array(&input), false);

        let input = [9, 7, 6, 2, 1];
        assert_eq!(is_gradually_decreasing_array(&input), false);

        let input = [1, 3, 2, 4, 5];
        assert_eq!(is_gradually_decreasing_array(&input), false);

        let input = [8, 6, 4, 4, 1];
        assert_eq!(is_gradually_decreasing_array(&input), false);

        let input = [1, 3, 6, 7, 9];
        assert_eq!(is_gradually_decreasing_array(&input), false);
    }

    #[test]
    fn test_5() {
        let input = vec![7, 6, 4, 2, 1];
        assert_eq!(part_2::is_gradually_decreasing_array(input), true);

        let input = vec![1, 2, 7, 8, 9];
        assert_eq!(part_2::is_gradually_decreasing_array(input), false);

        let input = vec![9, 7, 6, 2, 1];
        assert_eq!(part_2::is_gradually_decreasing_array(input), false);

        let input = vec![1, 3, 2, 4, 5];
        assert_eq!(part_2::is_gradually_decreasing_array(input), false);

        let input = vec![8, 6, 4, 4, 1];
        assert_eq!(part_2::is_gradually_decreasing_array(input), true);

        let input = vec![1, 3, 6, 7, 9];
        assert_eq!(part_2::is_gradually_decreasing_array(input), false);

        let input = vec![3, 2, 3, 4, 5];
        assert_eq!(part_2::is_gradually_increasing_array(input), true);

        let input = vec![3, 3, 3, 4, 5];
        assert_eq!(part_2::is_gradually_increasing_array(input), false);

        let input = vec![3, 4, 3, 5];
        assert_eq!(part_2::is_gradually_increasing_array(input), false);
    }

    #[test]
    fn test_6() {
        let input = vec![7, 6, 4, 2, 1];
        assert_eq!(part_2::is_gradually_increasing_array(input), false);

        let input = vec![1, 2, 7, 8, 9];
        assert_eq!(part_2::is_gradually_increasing_array(input), false);

        let input = vec![9, 7, 6, 2, 1];
        assert_eq!(part_2::is_gradually_increasing_array(input), false);

        let input = vec![1, 3, 2, 4, 5];
        assert_eq!(part_2::is_gradually_increasing_array(input), true);

        let input = vec![8, 6, 4, 4, 1];
        assert_eq!(part_2::is_gradually_increasing_array(input), false);

        let input = vec![1, 3, 6, 7, 9];
        assert_eq!(part_2::is_gradually_increasing_array(input), true);

        let input = vec![1, 3, 2, 3, 4];
        assert_eq!(part_2::is_gradually_increasing_array(input), true);

        let input = vec![62, 65, 67, 70, 73, 76, 75];
        assert_eq!(part_2::is_gradually_increasing_array(input), true);
    }
}

// between 424 and 447
