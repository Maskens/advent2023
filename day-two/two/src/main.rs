use regex::Regex;
use std::fs;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
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

    let mut sum = 0;

    for line in content.lines() {
        let mut largest_amount_of_each_color = vec![];

        let bag_content: Vec<Content> = get_content(line);

        for c in bag_content {
            update_colors(&mut largest_amount_of_each_color, c);
        }

        sum += calc_color_power(&largest_amount_of_each_color).unwrap_or(0);
    }

    println!("{}", sum);
}

fn calc_color_power(colors: &Vec<Content>) -> Option<u32> {
    colors.into_iter()
        .map(|c| c.amount)
        .reduce(|acc, e| acc * e)
}

fn update_colors(colors: &mut Vec<Content>, content: Content) {
    let c = colors.into_iter().find(|c| c.col == content.col);

    if let Some(color) = c {
        if color.amount < content.amount {
            color.amount = content.amount;
        }
    } else {
        colors.push(content);
    }
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
