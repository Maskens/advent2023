use std::fs;
use regex::Regex;

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE
}

struct Content {
    col: Color,
    amount: u32
}

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Could not read file!");

    let current_bag_blue = Content {
        col: Color::BLUE,
        amount: 14
    };

    let current_bag_red = Content {
        col: Color::RED,
        amount: 12
    };

    let current_bag_green = Content {
        col: Color::GREEN,
        amount: 13
    };

    let mut sum = 0;

    for line in content.lines() {
        let re = Regex::new(r"^Game\s(?<id>\d+):").unwrap();

        let game_id : u32 = re.captures(line).unwrap().name("id").unwrap()
            .as_str().parse().unwrap();

        let bag_content: Vec<Content> = get_content(line);

        let mut game_possible = true;
 
        for c in bag_content {
            game_possible = match c {
                Content { col: Color::GREEN, amount } 
                    => current_bag_green.amount >= amount,
                Content { col: Color::RED, amount } 
                    => current_bag_red.amount >= amount,
                Content { col: Color::BLUE, amount } 
                    => current_bag_blue.amount >= amount,
            };

            if !game_possible {
                break;
            }
        }

        if game_possible {
            sum += game_id;
        } else {
            println!("Game not possible!, Game id {}", game_id);
        }
    }

    println!("{}", sum);
}

fn get_content(line: &str) -> Vec<Content> {
    let re = Regex::new(r"(?<amount>\d+)\s(?<color>red|green|blue)").unwrap();
    return re.captures_iter(line)
        .map(|caps| {
            let amount: u32 = caps.name("amount").unwrap().as_str().parse().unwrap();
            let color: Color = match caps.name("color").unwrap().as_str() {
                "blue" => Color::BLUE,
                "red" => Color::RED,
                "green" => Color::GREEN,
                _ => Color::BLUE
            };

            Content {
                col: color,
                amount
            }
        }).collect();
}
