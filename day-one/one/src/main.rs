use std::fs;

fn main() {

    let content = fs::read_to_string("input2.txt")
        .expect("No file top read!");

    let mut sum = 0;

    for line in content.lines() {
        let mut first_digit: char = ' ';
        let mut last_digit: char = ' ';

        for char in line.chars() {
            if char.is_ascii_digit() {
                first_digit = char;
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_ascii_digit() {
                last_digit = char;
                break;
            }
        }

        let mut total_char = String::new();

        total_char.push(first_digit);
        total_char.push(last_digit);
        let total_char = total_char.trim();

        let total : u32 = total_char.parse()
            .expect("Could not parse total char");

        sum += total;
    }

    println!("{}", sum);
}
