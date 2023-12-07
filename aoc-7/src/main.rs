use phf::phf_map;
use std::collections::HashMap;

static CARD_VALUE_MAP: phf::Map<char, u32> = phf_map! {
    'J' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'T' => 0xa,
    'Q' => 0xc,
    'K' => 0xd,
    'A' => 0xe,
    'X' => 0xb, // for part one
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    c: String,
    cards: HashMap<char, u8>,
    hand_type: HandType,
    bet: u32,
    rank: u32,
}
impl Hand {
    pub fn new(cards: &str, bet: u32) -> Hand {
        let mut card_map: HashMap<char, u8> = HashMap::new();
        let mut rank = 0x00000;
        let mut jokers = 0;

        for (i, c) in cards.chars().rev().enumerate() {
            if let Some(x) = card_map.get_mut(&c) { 
                *x += 1;
            } else if c.eq(&'J') { 
                jokers += 1;
            }
            else {
                card_map.insert(c, 1);
            }
            let r = CARD_VALUE_MAP.get(&c).unwrap() << (i * 4);
            rank += r;
        }
        
        if card_map.is_empty() {
            card_map.insert('A', jokers);
            jokers = 0;
        }


        let mut hand_type: HandType;
        let hand_max = *card_map.values().max().unwrap();
        let hand_min = *card_map.values().min().unwrap();
        if hand_max == 5 { hand_type = HandType::FiveOfKind }
        else if hand_max == 1 { hand_type = HandType::HighCard }
        else if hand_max == 4 { hand_type = HandType::FourOfKind }
        else if hand_max == 3 {
            if hand_min == 2 { hand_type = HandType::FullHouse }
            else { hand_type = HandType::ThreeOfKind }
        }
        else if hand_max == 2 {
            let pairs = card_map.values().filter(|x| *x == &2);
            if pairs.count() == 2 { hand_type = HandType::TwoPairs }
            else { hand_type = HandType::OnePair }
        }
        else {
            panic!("Unrecognized hand set")
        }

        for _i in 0..jokers {
            hand_type = match hand_type {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfKind,
                HandType::TwoPairs => HandType::FullHouse,
                HandType::ThreeOfKind => HandType::FourOfKind,
                HandType::FullHouse => HandType::FourOfKind,
                HandType::FourOfKind => HandType::FiveOfKind,
                HandType::FiveOfKind => panic!("Should not be possible with jokers"),
            }
        }

        Hand {
            c: cards.to_string(),
            cards: card_map,
            hand_type,
            bet,
            rank,
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => self.rank.cmp(&other.rank),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("../../inputs/07/input.txt");

    let result_1 = process_hands(&input.replace('J', "X"));
    let result_2 = process_hands(input);

    println!("Result part one: {}", result_1);
    println!("Result part two: {}", result_2);
}

fn process_hands(input: &str) -> u64 {
    let mut hands = Vec::new();
    for line in input.lines() {
        let split = line.split_once(' ').unwrap();
        let bet = split.1.parse().unwrap();
        let hand = Hand::new(split.0, bet);
        hands.push(hand);
    }

    let mut result = 0;
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        let bet_value = (i + 1) as u64 * hand.bet as u64;
        result += bet_value;
    }
    result
}

