
fn main() {
    let mut lines = include_str!("../../inputs/06/input.txt").lines();
    let times_str = lines.next().unwrap().replace("Time:", "");
    let records_str = lines.next().unwrap().replace("Distance:", "");

    let times: Vec<u32> = times_str.trim().split(' ').filter(|item| !item.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
    let records: Vec<u32> = records_str.trim().split(' ').filter(|item| !item.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();

    let result_1 = part_one(&times, &records);
    let result_2 = part_two(&times, &records);

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);
}

fn part_one(times: &[u32], records: &[u32]) -> u32 {
    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let mut better_attempts = 0;
        for j in 0..*time {
            let distance = (time - j) * j; 
            if distance > records[i] {
                better_attempts += 1;
            }
        }
        result *= better_attempts;
    }
    result
}

fn part_two(times: &[u32], records: &[u32]) -> u64 {
    let time = times.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let record = records.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let mut better_attempts = 0;
    for j in 0..time {
        let distance = (time - j) * j; 
        if distance > record {
            better_attempts += 1;
        }
    }

    better_attempts
}