use std::{fs, ops::Add};

mod util;
mod grid;

use crate::util::Pos;
use crate::grid::Grid;


fn main() {
    let file_content = fs::read_to_string("input.txt")
        .expect("Could not open file");

    let mut grid = Grid::new();

    grid.populate(&file_content);

    grid.print();

    println!("Width: {}", grid.width());
    println!("Height: {}", grid.height());

    let mut numbers: Vec<u32> = vec![];

    for y in 0..grid.height() {
        let mut chars: Vec<char> = vec![];
        let mut is_number_valid = false;
        for x in 0..grid.width() {
            let c = grid.find_char(Pos { x: x as isize, y: y as isize }).unwrap();

            if c.is_ascii_digit() {
                chars.push(c);

                if !is_number_valid {
                    is_number_valid = grid.has_adjacent_symbol(Pos { x: x as isize, y: y as isize });
                }
            }

            if !c.is_ascii_digit() || x == grid.width() -1 {
                if !chars.is_empty() && is_number_valid {
                    // We have a valid number, store it
                    let mut number_string = String::new();
                    chars.iter().for_each(|c| number_string.push(c.clone()));

                    let number: u32 = number_string.parse().unwrap();
                    numbers.push(number);

                    is_number_valid = false;
                }
                chars.clear();
            }
        }
    }

    let sum: u32 = numbers.iter().sum();

    println!("{}", sum);
}
