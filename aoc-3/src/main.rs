#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Part {
    value: u32,
    position: Coordinates,
    length: i32,
}

struct CharPart {
    value: char,
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("../../inputs/03/input.txt");

    let (parts_coordinates, chars_coordinates) = fill_grid_data(input);

    let result_1 = process_parts(&parts_coordinates, &chars_coordinates);
    let result_2 = process_gears(&parts_coordinates, &chars_coordinates);

    println!("Part 1 result: {}", result_1);
    println!("Part 1 result: {}", result_2);
}

fn fill_grid_data(grid: &str) -> (Vec<Part>, Vec<CharPart>) {
    let mut parts_coordinates: Vec<Part> = Vec::new();
    let mut chars_coordinates: Vec<CharPart> = Vec::new();

    for (y, row) in grid.lines().enumerate() {
        let mut number_stack = String::new();
        for (x, c) in row.char_indices() {
            if c.is_ascii_digit() {
                number_stack.push(c);
            } else if c.ne(&'.') {
                chars_coordinates.push(CharPart {
                    value: c,
                    x: x as i32,
                    y: y as i32,
                });
            }

            if (!c.is_ascii_digit() || x == row.len() - 1) && !number_stack.is_empty() {
                let part = Part {
                    value: number_stack.parse().unwrap(),
                    position: Coordinates {
                        x: (x - number_stack.len()) as i32,
                        y: y as i32,
                    },
                    length: number_stack.len() as i32,
                };
                // println!("Part: {:?}", part);
                parts_coordinates.push(part);
                number_stack.clear();
            }
                
        }
    }

    (parts_coordinates, chars_coordinates)
}

fn process_gears(parts: &Vec<Part>, chars: &Vec<CharPart>) -> u32 {
    let mut gears_sum: u32 = 0;

    for char in chars {
        if char.value.ne(&'*') {
            continue;
        }

        let mut adjacent_parts: Vec<u32> = Vec::new();
        for part in parts {
            if is_part_touching_char(part, char) {
                adjacent_parts.push(part.value)
            } 
        }
        if adjacent_parts.len() == 2 {
            gears_sum += adjacent_parts[0] * adjacent_parts[1];
        }
    }

    gears_sum
}

fn process_parts(parts: &Vec<Part>, chars: &Vec<CharPart>) -> u32 {
    let mut parts_sum = 0;

    for part in parts {
        for char in chars {
            if char.y > part.position.y + 1 {
                break;
            }

            if is_part_touching_char(part, char) {
                parts_sum += part.value;
                break;
            } 
        }
    }

    parts_sum
}

fn is_part_touching_char(part: &Part, char_pos: &CharPart) -> bool {
    let part_pos = &part.position;
    if char_pos.y >= part_pos.y - 1
        && char_pos.y <= part_pos.y + 1
        && char_pos.x >= part_pos.x - 1
        && char_pos.x <= part_pos.x + part.length
    {
        return true;
    }
    false
}
