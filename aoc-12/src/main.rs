use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
    Operational,
    Damaged,
    Unknown,
}

fn main() {
    let input = include_str!("../../inputs/12/input.txt");

    let result_1 = process_line(input, 1);
    println!("Result 1: {}", result_1);
    
    let result_2 = process_line(input, 5);
    println!("Result 1: {}", result_2);
}

fn process_line(input: &str, repetitions: usize) -> u32 {
    let mut result = 0;

    for (_i, l) in input.lines().enumerate() {
        let record = l.split_once(' ').unwrap();
        let mut arrangements = HashSet::new();

        let spring_data: Vec<Operation> = record.0
            .chars()
            .map(|c| match c {
                '?' => Operation::Unknown,
                '#' => Operation::Damaged,
                '.' => Operation::Operational,
                _ => unreachable!(),
            })
            .collect();

        let groups: Vec<u32> = record.1   
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();


        let groups_repeated: Vec<u32> = groups    
            .iter()
            .cycle()
            .take(repetitions * groups.len())
            .cloned()
            .collect();

        let mut spring_data_duplicated = spring_data.clone();
        for _ in 1..repetitions {
            spring_data_duplicated.push(Operation::Unknown);
            spring_data_duplicated.append(&mut spring_data.clone());
        }
        process_spring(spring_data_duplicated, &groups_repeated, &mut arrangements);
        result += arrangements.len() as u32;
    }

    result
}

fn process_spring(spring_data: Vec<Operation>, groups: &[u32], arrangements: &mut HashSet<String>) {
    if !is_valid_configuration_partial(&spring_data, groups) {
        return;
    }
    let next_unknown = spring_data.iter().position(|op| op == &Operation::Unknown);

    if let Some(unknown_index) = next_unknown {
        let mut damaged_variant = spring_data.clone();
        damaged_variant[unknown_index] = Operation::Damaged;
        process_spring(damaged_variant, groups, arrangements);

        let mut operational_variant = spring_data.clone();
        operational_variant[unknown_index] = Operation::Operational;
        process_spring(operational_variant, groups, arrangements);
    }
    else {
        let s: String = operations_to_string(&spring_data);
        if is_valid_configuration(&s, groups) {
            arrangements.insert(s);
        }
    }
}

fn operations_to_string(spring_data: &[Operation]) -> String {
    let mut s: String = String::new();

    for op in spring_data.iter() {
        s.push(match op {
            Operation::Operational => '.',
            Operation::Damaged => '#',
            Operation::Unknown => '?',
        })
    }
    s
}

fn is_valid_configuration(spring_data: &str, group_count: &[u32]) -> bool {
    let groups: Vec<String> = spring_data
        .split('.')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if groups.len() != group_count.len() {
        return false;
    }

    for (i, g) in groups.iter().enumerate() {
        if g.len() != group_count[i] as usize {
            return false;
        }
    }

    true
}

fn is_valid_configuration_partial(spring_data: &[Operation], group_count: &[u32]) -> bool {
    let mut group_index = 0;
    let mut defective_counter = 0;

    for c in spring_data.iter() {
        if c.eq(&Operation::Damaged) {
            defective_counter += 1;
        }
        else if c.eq(&Operation::Operational) && defective_counter > 0 {
            if group_index >= group_count.len() {
                return false;
            }
            if defective_counter != group_count[group_index] {
                return false;
            }
            defective_counter = 0;
            group_index += 1;
        }
        
        else if c.eq(&Operation::Unknown) {
            if group_index >= group_count.len() {
                return true;
            }
            return defective_counter <= group_count[group_index];
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_valid_configuration;

    #[test]
    fn valid_configuration() {
        let spring_1 = ".#.###.#.######".to_string();
        let groups_1 = [1,3,1,6];

        let spring_2 = "####.#...#...".to_string();
        let groups_2 = [4,1,1];

        let spring_3 = "#.#.###".to_string();
        let groups_3 = [1,1,3];

        let spring_4 = ".#.#.#.#.######".to_string();
        let groups_4 = [1,3,1,6];

        let spring_5 = ".###.#.#.######".to_string();
        let groups_5 = [1,3,1,6];

        assert!(is_valid_configuration(&spring_1, &groups_1));
        assert!(is_valid_configuration(&spring_2, &groups_2));
        assert!(is_valid_configuration(&spring_3, &groups_3));
        assert!(!is_valid_configuration(&spring_4, &groups_4));
        assert!(!is_valid_configuration(&spring_5, &groups_5));
    }
}