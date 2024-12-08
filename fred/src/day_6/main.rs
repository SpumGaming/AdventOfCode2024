use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!("{}", part_1::do_work());
    println!("{}", part_2::do_work());
}

mod part_1 {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    enum Tile {
        Empty,
        Visited,
        Obstacle,
        Guard(Direction),
    }

    pub fn do_work() -> u32 {
        let file = File::open("src/day_6/input.txt").unwrap();
        let reader = io::BufReader::new(file);
        let mut grid: Vec<Vec<Tile>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|character| match character {
                        '.' => Tile::Empty,
                        '#' => Tile::Obstacle,
                        '^' => Tile::Guard(Direction::Up),
                        '>' => Tile::Guard(Direction::Right),
                        'v' => Tile::Guard(Direction::Down),
                        '<' => Tile::Guard(Direction::Left),
                        other => {
                            println!("{}", other);
                            panic!()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // grid.iter().for_each(|line| println!("\t{:?}\t", line));

        let mut guard_coords = get_guard_coords(&grid).unwrap();
        let mut direction = match grid[guard_coords.0][guard_coords.1] {
            Tile::Guard(Direction::Right) => Direction::Right,
            Tile::Guard(Direction::Left) => Direction::Left,
            Tile::Guard(Direction::Up) => Direction::Up,
            Tile::Guard(Direction::Down) => Direction::Down,
            _ => panic!(),
        };

        loop {
            if is_at_edge_of_map(&grid, guard_coords, direction) {
                grid[guard_coords.0][guard_coords.1] = Tile::Visited;
                break;
            }
            if will_walk_into_obstacle(&grid, guard_coords, direction) {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
            // check again then move and set previous coords to visited
            if is_at_edge_of_map(&grid, guard_coords, direction) {
                grid[guard_coords.0][guard_coords.1] = Tile::Visited;
                break;
            }

            grid[guard_coords.0][guard_coords.1] = Tile::Visited;
            guard_coords = match direction {
                Direction::Up => (guard_coords.0 - 1, guard_coords.1),
                Direction::Right => (guard_coords.0, guard_coords.1 + 1),
                Direction::Down => (guard_coords.0 + 1, guard_coords.1),
                Direction::Left => (guard_coords.0, guard_coords.1 - 1),
            };
        }

        let mut total_visited = 0;
        for line in grid {
            for tile in line {
                if tile == Tile::Visited {
                    total_visited += 1;
                }
            }
        }
        total_visited
    }

    fn get_guard_coords(grid: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
        for row in 0..grid.len() {
            for column in 0..grid[0].len() {
                if matches!(grid[row][column], Tile::Guard(_)) {
                    return Some((row, column));
                }
            }
        }
        None
    }

    fn will_walk_into_obstacle(
        grid: &Vec<Vec<Tile>>,
        coords: (usize, usize),
        direction: Direction,
    ) -> bool {
        let potential_next_coords = match direction {
            Direction::Up => (coords.0 - 1, coords.1),
            Direction::Down => (coords.0 + 1, coords.1),
            Direction::Right => (coords.0, coords.1 + 1),
            Direction::Left => (coords.0, coords.1 - 1),
        };
        grid
            .get(potential_next_coords.0)
            .unwrap()
            .get(potential_next_coords.1)
            .unwrap() == &Tile::Obstacle
    }

    fn is_at_edge_of_map(
        grid: &Vec<Vec<Tile>>,
        coords: (usize, usize),
        direction: Direction,
    ) -> bool {
        // dbg!(&grid);
        // dbg!(&coords);
        // dbg!(&direction);
        match direction {
            Direction::Up => {
                coords.0 == 0
            }
            Direction::Left => {
                coords.1 == 0
            }
            Direction::Down => {
                coords.0 == grid.len() - 1
            }
            Direction::Right => {
                coords.1 == grid[0].len() - 1
            }
        }
    }
}

mod part_2 {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    enum Tile {
        Empty,
        Visited,
        Obstacle,
        Guard(Direction),
    }

    pub fn do_work() -> u32 {
        let file = File::open("src/day_6/input.txt").unwrap();
        let reader = io::BufReader::new(file);
        let grid: Vec<Vec<Tile>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|character| match character {
                        '.' => Tile::Empty,
                        '#' => Tile::Obstacle,
                        '^' => Tile::Guard(Direction::Up),
                        '>' => Tile::Guard(Direction::Right),
                        'v' => Tile::Guard(Direction::Down),
                        '<' => Tile::Guard(Direction::Left),
                        other => {
                            println!("{}", other);
                            panic!()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // grid.iter().for_each(|line| println!("\t{:?}\t", line));
        let guard_initial_start_position = get_guard_coords(&grid).unwrap();

        let mut possible_obstacle_position_count: u64 = 0;
        for (row_index, row) in grid.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                let mut grid_clone = grid.clone();
                if (row_index, column_index) == guard_initial_start_position {
                    continue;
                }
                if grid_clone[row_index][column_index] != Tile::Empty {
                    continue;
                }
                grid_clone[row_index][column_index] = Tile::Obstacle;
                if is_infinite_loop(grid_clone) {
                    possible_obstacle_position_count += 1;
                }
            }
        }
        println!("answer: {}", possible_obstacle_position_count);
        possible_obstacle_position_count as u32
    }

    fn is_infinite_loop(mut grid_clone: Vec<Vec<Tile>>) -> bool {
        let mut is_infinite_loop = true;
        let mut guard_coords = get_guard_coords(&grid_clone).unwrap();
        let mut direction = match grid_clone[guard_coords.0][guard_coords.1] {
            Tile::Guard(Direction::Right) => Direction::Right,
            Tile::Guard(Direction::Left) => Direction::Left,
            Tile::Guard(Direction::Up) => Direction::Up,
            Tile::Guard(Direction::Down) => Direction::Down,
            _ => panic!(),
        };
        for _ in 0..10000 {
            if is_at_edge_of_map(&grid_clone, guard_coords, direction) {
                grid_clone[guard_coords.0][guard_coords.1] = Tile::Visited;
                is_infinite_loop = false;
                break;
            }

            // can have cases where you immediately need to turn multiple times
            for _ in 0..4 {
                if will_walk_into_obstacle(&grid_clone, guard_coords, direction) {
                    direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    }
                }
            }

            // check again then move and set previous coords to visited
            if is_at_edge_of_map(&grid_clone, guard_coords, direction) {
                grid_clone[guard_coords.0][guard_coords.1] = Tile::Visited;
                is_infinite_loop = false;
                break;
            }

            grid_clone[guard_coords.0][guard_coords.1] = Tile::Visited;
            guard_coords = match direction {
                Direction::Up => (guard_coords.0 - 1, guard_coords.1),
                Direction::Right => (guard_coords.0, guard_coords.1 + 1),
                Direction::Down => (guard_coords.0 + 1, guard_coords.1),
                Direction::Left => (guard_coords.0, guard_coords.1 - 1),
            };
        }
        is_infinite_loop
    }

    fn get_guard_coords(grid: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
        for row in 0..grid.len() {
            for column in 0..grid[0].len() {
                if matches!(grid[row][column], Tile::Guard(_)) {
                    return Some((row, column));
                }
            }
        }
        None
    }

    fn will_walk_into_obstacle(
        grid: &Vec<Vec<Tile>>,
        coords: (usize, usize),
        direction: Direction,
    ) -> bool {
        let potential_next_coords = match direction {
            Direction::Up => (coords.0 - 1, coords.1),
            Direction::Down => (coords.0 + 1, coords.1),
            Direction::Right => (coords.0, coords.1 + 1),
            Direction::Left => (coords.0, coords.1 - 1),
        };
        grid
            .get(potential_next_coords.0)
            .unwrap()
            .get(potential_next_coords.1)
            .unwrap() == &Tile::Obstacle
    }

    fn is_at_edge_of_map(
        grid: &Vec<Vec<Tile>>,
        coords: (usize, usize),
        direction: Direction,
    ) -> bool {
        // dbg!(&grid);
        // dbg!(&coords);
        // dbg!(&direction);
        match direction {
            Direction::Up => {
                coords.0 == 0
            }
            Direction::Left => {
                coords.1 == 0
            }
            Direction::Down => {
                coords.0 == grid.len() - 1
            }
            Direction::Right => {
                coords.1 == grid[0].len() - 1
            }
        }
    }
}
