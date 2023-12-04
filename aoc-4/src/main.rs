use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
struct Card {
    index: usize,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn main() {
    let input = include_str!("../../inputs/04/input.txt");

    let cards: Vec<Card> = input.lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line| line.split_once(" | ").unwrap())
        .map(|card| -> (HashSet<u32>, HashSet<u32>) {
            let left_numbers = card.0.split(' ').filter(|n| !n.is_empty()).map(|n| n.parse::<u32>().unwrap());
            let right_numbers = card.1.split(' ').filter(|n| !n.is_empty()).map(|n| n.parse::<u32>().unwrap());
            let winning_numbers = HashSet::from_iter(left_numbers);
            let my_numbers = HashSet::from_iter(right_numbers);

            (winning_numbers, my_numbers)
        })
        .enumerate()
        .map(|(i, card)| Card {
            index: i,
            winning_numbers: card.0,
            my_numbers: card.1,
        })
        .collect();

    let points: u32 = part_one(&cards);
    let scratchcards = part_two(&cards);
    
    println!("Result 1: {}", points);
    println!("Result 2: {}", scratchcards);
}

fn part_one(cards: &Vec<Card>) -> u32 {
    let mut points = 0;
    for card in cards {
        let winning_numbers = get_winning_numbers(card);
        if winning_numbers > 0 { points += 2_u32.pow(winning_numbers - 1) }
    }
    points
}

fn get_winning_numbers(card: &Card) -> u32 {
    card.my_numbers.intersection(&card.winning_numbers).count() as u32
}

fn part_two(cards: &Vec<Card>) -> u32 {
    let mut scratchcards = 0;
    let mut card_stack = VecDeque::from(cards.iter().collect::<Vec<&Card>>());
    while !card_stack.is_empty() {
        let card = card_stack.pop_front().unwrap();
        let winning_numbers = get_winning_numbers(card);
        let from_index = card.index + 1;
        let to_index = card.index + winning_numbers as usize + 1;
        for i in from_index..to_index {
            if i < cards.len() {
                card_stack.push_back(&cards[i]);
            }
        }
        scratchcards += 1;
    }

    scratchcards
}