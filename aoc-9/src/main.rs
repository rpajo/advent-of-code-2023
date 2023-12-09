fn main() {
    let input = include_str!("../../inputs/09/input.txt");

    let mut result = (0, 0);
    for l in input.lines() {
        let mut numbers: Vec<i32> = l.split(' ').map(|n| n.parse().unwrap()).collect();
        let mut last_elements = vec![*numbers.last().unwrap()];
        let mut first_elements = vec![*numbers.first().unwrap()];

        while !numbers.iter().all(|n| n.eq(&0)) {
            numbers = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();
            last_elements.push(*numbers.last().unwrap());
            first_elements.push(*numbers.first().unwrap());
        }

        let sum_back = last_elements
            .iter()
            .copied()
            .reduce(|acc, el| el + acc)
            .unwrap();
        let sum_front = first_elements
            .iter()
            .copied()
            .rev()
            .reduce(|acc, el| el - acc)
            .unwrap();

        result.0 += sum_back;
        result.1 += sum_front;
    }

    println!("Result 1: {}", result.0);
    println!("Result 2: {}", result.1);
}
