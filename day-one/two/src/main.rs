use std::fs;

struct  Number {
    char_val: char,
    str_value: String
}

fn main() {

    let numbers = [
        Number{
            char_val:'1',
            str_value: String::from("one")
        },
        Number {
            char_val: '2', 
            str_value: String::from("two")        
        },
        Number {
            char_val: '3', 
            str_value: String::from("three")        
        },
        Number {
            char_val: '4', 
            str_value: String::from("four")        
        },
        Number {
            char_val: '5', 
            str_value: String::from("five")        
        },
        Number {
            char_val: '6', 
            str_value: String::from("six")        
        },
        Number {
            char_val: '7', 
            str_value: String::from("seven")        
        },
        Number {
            char_val: '8', 
            str_value: String::from("eight")        
        },
        Number {
            char_val: '9', 
            str_value: String::from("nine")        
        },
    ];


    let content = fs::read_to_string("input2.txt")
        .expect("No file top read!");


    let mut sum = 0;

    for line in content.lines() {
        println!("{}", line);

        let string_number_optional: Option<(char, usize)> = 
            try_find_left_string_number_in_line(line, &numbers);
        let char_number_optional: Option<(char, usize)> = 
            try_find_left_number_in_line(&line);

        let leftmost_char = match (string_number_optional, char_number_optional) {
            (Some((number, _)), None) => number,
            (None, Some((number, _))) => number,
            (Some((number1, index1)), Some((number, index))) => {
                if index1 < index { number1 } else { number }
            },
            (None, None) => ' ' 
        };

        let string_number_optional: Option<(char, usize)> = 
            try_find_right_string_number_in_line(line, &numbers);
        let char_number_optional: Option<(char, usize)> = 
            try_find_right_number_in_line(&line);

        let rightmost_char = match (string_number_optional, char_number_optional) {
            (Some((number, _)), None) => number,
            (None, Some((number, _))) => number,
            (Some((number1, index1)), Some((number, index))) => {
                if index1 > index { number1 } else { number }
            },
            (None, None) => ' ' 
        };

        let char_number = format!("{}{}", leftmost_char, rightmost_char);

        let total : u32 = char_number.parse()
            .expect("Could not parse total char");

        println!("{}", total);

        sum += total;

    }

    println!("{}", sum);
}

fn try_find_right_number_in_line(line: &str) -> Option<(char, usize)> {
    line.chars().rev().enumerate() // Todo: Här blir det fel!!
        .map(|(index, c)| (c, index))
        .find(|(c, _)| c.is_numeric())
        .map(|(c, index)| (c, line.len() - 1 - index))
}

fn try_find_right_string_number_in_line(line: &str, numbers: &[Number]) -> Option<(char, usize)> {
    let mut matched_number: Option<char> = None;

    let mut index = 0;

    for num in numbers.iter() {
        if let Some(n) = line.rfind(&num.str_value.to_string()) { // Howto handle "fivefive" ??

            if n >= index {
                index = n;
                matched_number = Some(num.char_val);
            }
        }
    }

    return match matched_number {
        Some(_) => Some((matched_number.unwrap(), index)),
        None => None
    }
}

fn try_find_left_number_in_line(line: &str) -> Option<(char, usize)> {
    return line.chars().enumerate()
        .map(|(index, c)| (c, index))
        .find(|(c, _)| c.is_numeric());
}

fn try_find_left_string_number_in_line(line: &str, numbers: &[Number]) -> Option<(char, usize)> {
    // println!("On line {}", line);
    let mut matched_number: Option<char> = None;

    let mut index = line.len();

    for num in numbers {
        if let Some(n) = line.find(&num.str_value.to_string()) {

            if n <= index {
                index = n;
                matched_number = Some(num.char_val);
            }
        }
    }

    return match matched_number {
        Some(_) => Some((matched_number.unwrap(), index)),
        None => None
    }
}


