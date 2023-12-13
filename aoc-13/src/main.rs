fn main() {
    let input = include_str!("../../inputs/13/input.txt");

    let result_1 = find_mirrors(input, false);
    let result_2 = find_mirrors(input, true);

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);
}

fn find_mirrors(input: &str, fix_smudge: bool) -> usize {
    let mut pattern: Vec<String> = Vec::new();

    let mut reflection_sum = 0;
    for i in 0..input.lines().count() + 1 {
        let line = input.lines().nth(i).unwrap_or_default();
        if line.is_empty() || i == input.lines().count(){
            let mut mirror = process_pattern(&pattern, fix_smudge);
            if let Some(mirror_index) = mirror {
                reflection_sum += (mirror_index + 1) * 100
            }
            else {
                let transposed = flip_pattern(&pattern);
                mirror = process_pattern(&transposed, fix_smudge);
                if let Some(mirror_index) = mirror {
                    reflection_sum += mirror_index + 1;
                }
            }

            pattern.clear();
            continue;
        }
        pattern.push(line.to_string());
    }
    reflection_sum
}

fn process_pattern(pattern: &[String], fix_smudges: bool) -> Option<usize> {
    // println!("Process pattern");
    // print_pattern(pattern);

    let mut mirror_index: Option<usize> = None;
    for i in 0..pattern.len() - 1 {
        let is_mirrored = compare_slice(pattern, (i, i + 1), fix_smudges);
        if is_mirrored { 
            mirror_index = Some(i);
            break;
        }
    }

    mirror_index
}

fn compare_slice(pattern: &[String], indexes: (usize, usize), has_smudge: bool) -> bool {
    let mut smudge = has_smudge; 
    let diffs = get_string_diffs(&pattern[indexes.0], &pattern[indexes.1]);
    
    if pattern[indexes.0] != pattern[indexes.1] {
        if  smudge && diffs == 1 {
            smudge = false;
        }
        else {   
            return false;
        }
    }

    if indexes.0 == 0 || indexes.1 == pattern.len() - 1 {
        return !smudge;
    }

    compare_slice(pattern, (indexes.0 - 1, indexes.1 + 1), smudge)
}

fn flip_pattern(pattern: &[String]) -> Vec<String> {
    let max_length = pattern[0].len(); // Assuming all rows have the same length
    let mut transposed: Vec<String> = Vec::new();
    for i in 0..max_length {
        let column: String = pattern.iter().map(|row| row.chars().nth(i).unwrap()).collect();
        transposed.push(column);
    }
    transposed
}

fn get_string_diffs(a: &str, b: &str) -> u8 {
    let mut diffs = 0;
    for (i, c) in a.chars().enumerate() {
        if !c.eq(&b.chars().nth(i).unwrap()) {
            diffs += 1;
        }
    }
    diffs
} 