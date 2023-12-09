use std::collections::HashMap;

struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("../../inputs/08/input.txt");
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    lines.next();

    let mut node_map: HashMap<String, Node> = HashMap::new();
    for l in lines {
        let s = l.split_once(" = ").unwrap();
        let id = s.0.to_string();

        let left_right = s.1[1..s.1.len() - 1].split_once(", ").unwrap();
        let node = Node {
            id: id.clone(),
            left: left_right.0.to_string(),
            right: left_right.1.to_string(),
        };
        node_map.insert(id, node);
    }

    let result_1 = part_one(&node_map, directions);
    let result_2 = part_two(&node_map, directions);
    println!("Result 1: {} steps", result_1);
    println!("Result 2: {} steps", result_2);
}

fn part_one(node_map: &HashMap<String, Node>, directions: &str) -> u32 {
    let mut at_node = node_map.get("AAA").unwrap();
    let mut steps = 0;
    while at_node.id.ne("ZZZ") {
        let dir = directions.chars().nth(steps % directions.len()).unwrap();
        at_node = match dir {
            'L' => node_map.get(&at_node.left).unwrap(),
            'R' => node_map.get(&at_node.right).unwrap(),
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps as u32
}

fn part_two(node_map: &HashMap<String, Node>, directions: &str) -> u128 {
    let mut at_nodes: Vec<&Node> = node_map.values().filter(|n| n.id.ends_with('A')).collect();

    let mut steps = 0;
    let mut steps_to_z = vec![0; at_nodes.len()];

    while steps_to_z.iter().any(|n| n.eq(&0)) {
        at_nodes = at_nodes
            .iter()
            .enumerate()
            .map(|(i, n)| -> &Node {
                let dir = directions.chars().nth(steps % directions.len()).unwrap();
                let new = match dir {
                    'L' => node_map.get(&n.left).unwrap(),
                    'R' => node_map.get(&n.right).unwrap(),
                    _ => unreachable!(),
                };
                if new.id.chars().last().unwrap().eq(&'Z') {
                    steps_to_z[i] = (steps + 1) as u128;
                }
                new
            })
            .collect();
        steps += 1;
    }
    lcm_of_vector(&steps_to_z)

}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
fn lcm_of_vector(vec: &[u128]) -> u128 {
    vec.iter().cloned().fold(1, lcm)
}
