use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let input = include_str!("../../inputs/16/input.txt");

    let grid: Vec<Vec<(char, Vec<bool>)>> = input
        .lines()
        .map(|l| l.chars()
            .map(|c| (c, (vec![false; 4])))
            .collect()
        )
        .collect();

    let result_1 = process_beam(&grid, (0, 0), Direction::Right);
    
    let mut result_2 = 0;
    let grid_width = grid[0].len() as i32;
    let grid_height = grid.len() as i32;
    for y in 0..grid_height {
        let tiles_energized_1 = process_beam(&grid, (y, 0), Direction::Right);
        let tiles_energized_2 = process_beam(&grid, (y, grid_width - 1), Direction::Left);
        
        if result_2 < tiles_energized_1 {
            result_2 = tiles_energized_1;
        }
        if result_2 < tiles_energized_2 {
            result_2 = tiles_energized_2;
        }
    }
    for x in 0..grid_width {
        let tiles_energized_1 = process_beam(&grid, (0, x), Direction::Down);
        let tiles_energized_2 = process_beam(&grid, (0, grid_height - 1), Direction::Up);
        
        if result_2 < tiles_energized_1 {
            result_2 = tiles_energized_1;
        }
        if result_2 < tiles_energized_2 {
            result_2 = tiles_energized_2;
        }
    }

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);

}

fn process_beam(grid_source: &Vec<Vec<(char, Vec<bool>)>>, beam_position: (i32, i32), beam_dir: Direction) -> i32 {
    let mut grid = grid_source.clone();
    let mut beams: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();
    beams.push_back((beam_position, get_direction_vec(beam_dir)));

    // print_grid(&grid);

    let mut tiles_energized = 0;
    while let Some(beam) = beams.pop_front() {
        let beam_pos = beam.0;
        let beam_direction = get_direction_enum(&beam.1);
        let beam_direction_vec = beam.1;

        if beam_pos.0 < 0 || beam_pos.0 == grid.len() as i32 {
            continue;
        }
        if beam_pos.1 < 0 || beam_pos.1 == grid[0].len() as i32 {
            continue;
        }

        let grid_data = grid[beam_pos.0 as usize][beam_pos.1 as usize].clone();
        let visited = grid_data.1.iter().any(|v| *v);
        let visited_index = match beam_direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Right => 2,
            Direction::Left => 3,
        };
        if visited {
            if grid_data.1[visited_index] {
                continue;
            }
        }
        else {
            grid[beam_pos.0 as usize][beam_pos.1 as usize].1[visited_index] = true;
            tiles_energized += 1;
        }

        match grid[beam_pos.0 as usize][beam_pos.1 as usize].0 {
            '.' => { beams.push_back((add_positions(&beam_pos, &beam_direction_vec), beam_direction_vec)); },
            '|' => {
                if beam_direction == Direction::Left || beam_direction == Direction::Right {
                    let up_vec = get_direction_vec(Direction::Up);
                    let down_vec = get_direction_vec(Direction::Down);
                    beams.push_back((add_positions(&beam_pos, &up_vec), up_vec));
                    beams.push_back((add_positions(&beam_pos, &down_vec), down_vec));
                }
                else {
                    beams.push_back((add_positions(&beam_pos, &beam_direction_vec), beam_direction_vec));
                }
            },
            '-' => {
                if beam_direction == Direction::Up || beam_direction == Direction::Down {
                    let left_vec = get_direction_vec(Direction::Left);
                    let right_vec = get_direction_vec(Direction::Right);
                    beams.push_back((add_positions(&beam_pos, &left_vec), left_vec));
                    beams.push_back((add_positions(&beam_pos, &right_vec), right_vec));
                }
                else {
                    beams.push_back((add_positions(&beam_pos, &beam_direction_vec), beam_direction_vec));
                }
            },
            '\\' => {
                let v = match beam_direction {
                    Direction::Up => get_direction_vec(Direction::Left),
                    Direction::Down => get_direction_vec(Direction::Right),
                    Direction::Right => get_direction_vec(Direction::Down),
                    Direction::Left => get_direction_vec(Direction::Up),
                };
                beams.push_back((add_positions(&beam_pos, &v), v));
            },
            '/' => {
                let v = match beam_direction {
                    Direction::Up => get_direction_vec(Direction::Right),
                    Direction::Down => get_direction_vec(Direction::Left),
                    Direction::Right => get_direction_vec(Direction::Up),
                    Direction::Left => get_direction_vec(Direction::Down),
                };
                beams.push_back((add_positions(&beam_pos, &v), v));
            },
            _ => unreachable!()
        }
    }
    tiles_energized
}

fn get_direction_vec(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up =>(-1, 0),
        Direction::Down => (1, 0),
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
    }
}
fn get_direction_enum(direction: &(i32, i32)) -> Direction {
    match direction {
        (-1, 0) => Direction::Up,
        (1, 0) => Direction::Down,
        (0, 1) => Direction::Right,
        (0, -1) => Direction::Left,
        _ => unreachable!()
    }
}
fn add_positions(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}