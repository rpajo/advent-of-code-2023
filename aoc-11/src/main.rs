fn main() {
    let input = include_str!("../../inputs/11/input.txt");
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut empty_columns: Vec<usize> = Vec::new();

    for i in 0..lines[0].len() {
        let is_empty = lines
            .iter()
            .map(|c| c[i])
            .all(|c| c.eq(&'.'));
        if is_empty {
            empty_columns.push(i);
        }
    }

    let shortest_paths_1 = process_galaxies(&lines, &empty_columns, 1);
    let shortest_paths_2 = process_galaxies(&lines, &empty_columns, 1000000);

    let result_1: u64 = shortest_paths_1.iter().sum();
    let result_2: u64 = shortest_paths_2.iter().sum();
    
    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);
}

fn process_galaxies(lines: &[Vec<char>], empty_columns: &[usize], multiplier: u64) -> Vec<u64> {
    let mut galaxies: Vec<(u64, u64)> = Vec::new();

    let mut empty_rows = 0;
    for (y, line) in lines.iter().enumerate() {
        if !line.contains(&'#') {
            empty_rows += 1;
            continue;
        }

        for (x, c) in line.iter().enumerate() {
            if c.eq(&'#') {
                let empty_cols = empty_columns.iter().filter(|e| *e < &x).count();
                galaxies.push((
                    ((y - empty_rows) + empty_rows * multiplier as usize) as u64,
                    ((x - empty_cols) + empty_cols * multiplier as usize) as u64
                ));
            }   
        }
    }

    let mut shortest_paths = Vec::new();
    for (i, galaxy_1) in galaxies.iter().enumerate() {
        for (_j, galaxy_2) in galaxies.iter().enumerate().skip(i+1) {
            let distance = galaxy_1.0.abs_diff(galaxy_2.0) + galaxy_1.1.abs_diff(galaxy_2.1);
            shortest_paths.push(distance);
        }
    }
    shortest_paths
}
