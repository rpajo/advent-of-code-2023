
fn main() {
    let lines: Vec<&str> = include_str!("../../inputs/05/input.txt").lines().collect();

    let seed_input: Vec<u64> = lines[0]
        .replace("seeds: ", "")
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();    

    let mut main_map = Vec::new();
    let mut seed_iter = seed_input.iter();
    
    let mut min_location: u64 = 0;
    while let Some(seed) = seed_iter.next() {
        let seed_range = seed_iter.next().unwrap();
        
        main_map = (*seed..(seed+seed_range)).collect();
        let new_map: Vec<u64> = process_seeds(main_map, &lines[2..]);
        let min_from_batch = new_map.iter().min().unwrap();
        if min_location == 0 || *min_from_batch < min_location {
            min_location = *min_from_batch;
        }
    }
    println!("Min location: {}", min_location);



}

fn process_seeds(mut main_map: Vec<u64>, lines: &[&str]) -> Vec<u64> {
    println!("Process {} seeds", main_map.len());
    let mut new_map: Vec<u64> = vec![0; main_map.len()];
    for (i, line) in lines.iter().enumerate() {
        if line.contains("map:") {
            println!("{}", line);
            continue;
        }

        if line.is_empty() {
            // println!("{:?} -> {:?}\n", main_map, new_map);
            main_map = new_map;
            new_map = vec![0; main_map.len()];
            continue;
        }

        let mapping: Vec<u64> = line
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        for (i, item) in main_map.iter().enumerate() {
            let new = map_item(*item, &mapping);
            if new_map[i].eq(&0) || new.ne(item) {
                new_map[i] = new;
            }
        }
    }
    new_map
}

fn map_item(item: u64, map: &[u64]) -> u64 {
    if item >= map[1] && item < map[1] + map[2] {
        return map[0] + item - map[1];
    }

    item
}
