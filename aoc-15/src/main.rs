use std::collections::{VecDeque, HashMap};

#[derive(Debug)]
struct Step {
    box_index: u8,
    operation: Operation,
    label: String
}

struct BoxLense {
    focal_length: u8,
    label: String
}

#[derive(Debug)]
enum Operation {
    Remove(),
    Insert(u8)
}

fn main() {
    let input = include_str!("../../inputs/15/test.txt");
    
    let result_1: u32 = input
        .trim()
        .split(',')
        .map(hash_string)
        .sum();
        
    println!("Result 1: {}", result_1);

    let data: Vec<Step> = input
        .trim()
        .split(',')
        .map(|seq| {
            if let Some(remove_op) = seq.split_once('-') {
                Step {
                    box_index: hash_string(remove_op.0) as u8,
                    label: remove_op.0.to_string(),
                    operation: Operation::Remove()
                }
            }
            else if let Some(insert_op) = seq.split_once('=') {
                Step {
                    box_index: hash_string(insert_op.0) as u8,
                    label: insert_op.0.to_string(),
                    operation: Operation::Insert(insert_op.1.parse::<u8>().unwrap())
                }
            }
            else {
                unreachable!();
            }
        }
        ).collect();

    let mut boxes: HashMap<u8, Vec<Option<BoxLense>>> = HashMap::new();

    for step in data {
        println!("\n{:?}", step);
        boxes.entry(step.box_index).or_default();
        let selected_box = boxes.get_mut(&step.box_index).unwrap();

        match step.operation {
            Operation::Remove() => {
                let lense_pos = selected_box
                    .iter()
                    .filter_map(|lense| lense.as_ref())
                    .position(|lense| lense.label == step.label);
                if let Some(lense_index) = lense_pos {
                    println!("Remove lense {} from box {}", step.label, step.box_index);
                    selected_box[lense_index] = None;
                } 
                else {
                    println!("Lense with label {} not in box {}", step.label, step.box_index);
                }
            },
            Operation::Insert(focal_length) => {
                let lense_pos = selected_box
                    .iter()
                    .position(|lense| {
                        // println!("{} = {} : {}", lense.label, step.label,  lense.label == step.label);
                        if let Some(l) = lense {
                            l.label == step.label
                        }
                        else {
                            false
                        }
                    });
                if let Some(lense_index) = lense_pos {
                    let existing_lense = selected_box[lense_index].as_mut().unwrap();
                    println!("Exchange lense {}: {} -> {}", existing_lense.label, existing_lense.focal_length, focal_length);
                    existing_lense.focal_length = focal_length;
                } 
                else {
                    println!("Insert new lense {} ({}) into box {}", step.label, focal_length, step.box_index);
                    selected_box.push(Some(BoxLense {
                        focal_length,
                        label: step.label.clone()
                    }));
                }
            },
        }
    }  

    let mut focusing_power = 0;
    println!("\nPRINT BOXES");
    for (i, b) in boxes {
        // println!("{}", i);
        if b.iter().all(|x| x.is_none()) {
            continue;
        }
        println!("Box {}", i);
        for (j, l) in b.iter().filter(|l| l.is_some()).enumerate() {
            let lense = l.as_ref().unwrap();
            let fp = (i as u32 + 1) * (j as u32 + 1) * lense.focal_length as u32;
            println!("Lense {}, fl: {} -> {}", lense.label, lense.focal_length, fp);

            focusing_power += fp;
        }
    }

    println!("Result 2: {}", focusing_power);


}

fn hash_string(s: &str) -> u32 {
    s.chars()
        .map(|c| {c as u32})
        .fold(0, |acc, x| ((acc + x) * 17) % 256)
}