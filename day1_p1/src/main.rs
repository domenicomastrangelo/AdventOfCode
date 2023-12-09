use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let lines = BufReader::new(file).lines();
    let mut sum = 0;

    lines.into_iter().for_each(|line| {
        let line = match line {
            Ok(line) => line,
            Err(_) => "".to_string(),
        };

        let number = find_number_in_line(line);

        sum += number;
    });

    println!("Sum: {}", sum);
}

fn find_number_in_line(line: String) -> i32 {
    let mut first_number: Option<u8> = None;
    let mut last_number: Option<u8> = None;

    let mut first_number_position: usize = 0;
    let mut current_position: usize = 0;

    for ch in line.chars() {
        if ch.is_digit(10) && first_number.is_none() {
            first_number = Some(ch as u8);
            last_number = Some(ch as u8);

            first_number_position = current_position;
        } else if ch.is_digit(10) && first_number_position < current_position {
            last_number = Some(ch as u8);
        }

        current_position += 1;
    }

    let mut number = "".to_string();

    match (first_number, last_number) {
        (Some(first_number), Some(last_number)) => {
            number.push(first_number as char);
            number.push(last_number as char);
        }
        _ => (),
    }

    let number = match number.parse::<i32>() {
        Ok(number) => number,
        Err(l) => {
            println!("Error: {}", l);
            0
        }
    };
}
