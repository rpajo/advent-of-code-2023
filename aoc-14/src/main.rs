use std::{fmt, collections::HashMap};

#[derive(PartialEq, Eq)]
enum Cell {
    Rock,
    FixedRock,
    Empty,
}
enum TiltDirection {
    North,
    West,
    South,
    East,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Rock => write!(f, "⊛"),
            Cell::FixedRock => write!(f, "⊟"),
            Cell::Empty => write!(f, "."),
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/14/test.txt");

    let mut rocks = Vec::new();
    let mut platform: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Cell::Rock,
                    '.' => Cell::Empty,
                    '#' => Cell::FixedRock,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    for (y, row) in platform.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.eq(&Cell::Rock) {
                rocks.push((y as i32, x as i32));
            }
        }
    }

    let mut hash_table = HashMap::new();
    let cycles = 1_000_000_000;
    let mut cycles_remaining = None;
    let mut i = 1;
    loop {
        tilt_platform(&mut platform, &mut rocks, TiltDirection::North);
        tilt_platform(&mut platform, &mut rocks, TiltDirection::West);
        tilt_platform(&mut platform, &mut rocks, TiltDirection::South);
        tilt_platform(&mut platform, &mut rocks, TiltDirection::East);
        
        let hash: String = platform
            .iter()
            .flat_map(|row| row.iter().map(|c| c.to_string()))
            .collect();

        if let Some(remaining) = cycles_remaining {
            if remaining == 0 {
                break;
            }
            cycles_remaining = Some(remaining - 1);

        }
       else {
            if hash_table.contains_key(&hash) {
                // break;
                let loop_start = *hash_table.get(&hash).unwrap();
                let loop_size = i - loop_start;
                let rem = (cycles - loop_start - 1) % loop_size;
                cycles_remaining = Some(rem);
            }
            hash_table.insert(hash, i);
        }
        i += 1;
    }

    let mut result_1 = 0;
    for (i, row) in platform.iter().rev().enumerate() {
        let rocks = row.iter().filter(|cell| *cell == &Cell::Rock).count();
        result_1 += rocks * (i + 1);
    }
    
    println!("Result: {}", result_1);
}

fn tilt_platform(platform: &mut Vec<Vec<Cell>>, rocks: &mut Vec<(i32, i32)>, tilt_direction: TiltDirection) {
    let platform_width = platform[0].len() as i32;
    let platform_height = platform.len() as i32;
    let movement_vector: (i32, i32);
    match tilt_direction {
        TiltDirection::North => {
            movement_vector = (-1, 0); 
            rocks.sort_by(|a, b| a.0.cmp(&b.0))
        },
        TiltDirection::South => {
            movement_vector = (1, 0);
            rocks.sort_by(|a, b| b.0.cmp(&a.0))
        },
        TiltDirection::West => {
            movement_vector = (0, -1);
            rocks.sort_by(|a, b| a.1.cmp(&b.1))
        },
        TiltDirection::East => {
            movement_vector = (0, 1);
            rocks.sort_by(|a, b| b.1.cmp(&a.1))
        },
    };
    for rock in rocks {
        platform[rock.0 as usize][rock.1 as usize] = Cell::Empty;
        let mut position = *rock;
        loop {
            let new_position = add_positions(&position, &movement_vector);

            if (new_position.0 < 0 || new_position.0 == platform_width) ||
                (new_position.1 < 0 || new_position.1 == platform_height) ||
                platform[new_position.0 as usize][new_position.1 as usize] != Cell::Empty {
                break;
            }
            position = new_position;
        }
        platform[position.0 as usize][position.1 as usize] = Cell::Rock;
        *rock = position;
    }
}

fn add_positions(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
