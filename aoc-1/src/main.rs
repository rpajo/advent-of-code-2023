fn main() {
    let input = include_str!("../../inputs/01/input.txt");

    let result: u32 = input.lines()
        .map(|line| line.trim())
        .map(replace_with_numbers)
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect())
        .map(|chars: Vec<char>| -> String {
            let mut first_and_last = String::new();
            first_and_last.push(*chars.first().unwrap());
            first_and_last.push(*chars.last().unwrap());
            first_and_last
        } )
        .map(|num| num.parse::<u32>().unwrap_or_default())
        .sum();

    println!("Result {}", result);
}

fn replace_with_numbers(line: &str) -> String {
    line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
   
}