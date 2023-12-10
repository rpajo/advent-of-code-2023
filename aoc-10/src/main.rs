use std::{collections::VecDeque, vec};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Debug)]
struct Pipe {
    pos: Position,
    steps: u32,
}

fn main() {
    let input = include_str!("../../inputs/10/input.txt");

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut step_grid: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];

    let s_position = input
        .replace(['\n', '\r'], "")
        .char_indices()
        .find(|c| c.1.eq(&'S'))
        .map(|x| x.0)
        .unwrap();
    
    let start = Position {
        y: s_position / grid[0].len(),
        x: s_position % grid[0].len(), 
    };
    step_grid[start.y][start.x] = 0;

    let start_connected: Vec<Pipe> = get_connected_positions(&start, &grid)
        .iter()
        .map(|p| Pipe { pos: *p, steps: 1 })
        .collect();

    let mut position_stack: VecDeque<Pipe> = VecDeque::from(start_connected);
    while !position_stack.is_empty() {
        let pipe = position_stack.pop_front().unwrap();
        if step_grid[pipe.pos.y][pipe.pos.x] > 0 && step_grid[pipe.pos.y][pipe.pos.x] <= pipe.steps as i32 {
            continue;
        }
        step_grid[pipe.pos.y][pipe.pos.x] = pipe.steps as i32;
        let connected = get_connected_positions(&pipe.pos, &grid);
        
        for c in connected {
            position_stack.push_back(Pipe { pos: c, steps: pipe.steps + 1 });
        }
    }



    let mut enclosed_counter: u32 = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut opened: bool = false;
        for (j, c) in row.iter().enumerate() {
            if ['|', 'F', '7'].contains(c) && step_grid[i][j] >= 0 {
                opened = !opened;
            }
            else if opened && step_grid[i][j] < 0 {
                enclosed_counter += 1;
            }
        }
    }

    let result_1 = *step_grid.iter().flatten().max().unwrap();

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", enclosed_counter);


}

fn get_connected_positions(pos: &Position, grid: &Vec<Vec<char>>) -> Vec<Position> {
    let north_connected = ['|', 'F', '7'];
    let south_connected = ['|', 'L', 'J'];
    let west_connected = ['-', 'F', 'L'];
    let east_connected = ['-', 'J', '7'];

    let c = grid[pos.y][pos.x];

    let mut connected = Vec::new();
    if pos.y > 0 && ['S', '|', 'J', 'L'].contains(&c) && north_connected.contains(grid[pos.y - 1].get(pos.x).unwrap()) {
        connected.push(Position { y: pos.y - 1, x: pos.x });
    }
    if pos.y < grid.len() - 1 && ['S', '|', 'F', '7'].contains(&c) && south_connected.contains(grid[pos.y + 1].get(pos.x).unwrap()) {
        connected.push(Position { y: pos.y + 1, x: pos.x });
    }
    if pos.x > 0 && ['S', '-', '7', 'J'].contains(&c) && west_connected.contains(grid[pos.y].get(pos.x - 1).unwrap()) {
        connected.push(Position { y: pos.y, x: pos.x - 1 });
    }
    if pos.x < grid[0].len() - 1 && ['S', '-', 'F', 'L'].contains(&c) && east_connected.contains(grid[pos.y].get(pos.x + 1).unwrap()) {
        connected.push(Position { y: pos.y, x: pos.x + 1 });
    }

    connected
}
