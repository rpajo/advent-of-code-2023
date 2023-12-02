use {
    once_cell::sync::Lazy,
    regex::Regex,
};

#[derive(Debug)]
enum BagItems {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct GameRound {
    id: u32,
    rounds: Vec<Vec<BagItems>>,
    valid: bool, 
    power: u32
}

struct GameLimits {
    red: u32,
    blue: u32,
    green: u32,
}

fn main() {
    let input: &str = include_str!("../../inputs/02/input.txt");

    let limits = GameLimits {
        red: 12,
        green: 13,
        blue: 14
    }; 

    let mut game_sum = 0;
    let mut game_power = 0;
    for line in input.lines() {
        let mut game = parse_line(line);

        let mut min_items = GameLimits {
            red: 0,
            green: 0,
            blue: 0
        }; 
        for round in game.rounds {
            for item in round {
                match item {
                    BagItems::Red(x) => {
                        if x > limits.red { game.valid = false }
                        if x > min_items.red { min_items.red = x }
                    },
                    BagItems::Green(x) => {
                        if x > limits.green { game.valid = false }
                        if x > min_items.green { min_items.green = x }
                    },
                    BagItems::Blue(x) => {
                        if x > limits.blue { game.valid = false }
                        if x > min_items.blue { min_items.blue = x }
                    },
                }
            }
        }
        if game.valid {
            game_sum += game.id;
        }

        game.power = min_items.blue * min_items.green * min_items.red;
        game_power += game.power; 
    }

    println!("Part 1 {}", game_sum);
    println!("Part 2 {}", game_power);

}


fn parse_line(line: &str) -> GameRound {
    static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?<id>\d+): ").unwrap());
    let game_id = &REG.captures(&line).unwrap()["id"];
    
    let mut game = GameRound {
        id: game_id.parse::<u32>().unwrap(),
        rounds: Vec::new(), 
        valid: true,
        power: 0
    };
    
    let game_data = REG.replace(line, "");
    let rounds = game_data.split("; ");

    for round in rounds {
        let cubes: Vec<BagItems> = round.split(", ")
            .map(|c| -> BagItems {
                let items: Vec<&str> = c.split(' ').collect();
                match items[1] {
                    "red" => BagItems::Red(items[0].parse::<u32>().unwrap()),
                    "blue" => BagItems::Blue(items[0].parse::<u32>().unwrap()),
                    "green" => BagItems::Green(items[0].parse::<u32>().unwrap()),
                    _ => panic!()
                }
            })
            .collect();
        game.rounds.push(cubes);
        
    }
    game
}