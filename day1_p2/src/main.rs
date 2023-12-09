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

    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    lines.into_iter().for_each(|line| {
        let line = match line {
            Ok(line) => line,
            Err(_) => "".to_string(),
        };

        let mut first_number = "".to_string();
        let mut second_number = "".to_string();

        line.chars().enumerate().for_each(|(i, c)| {
            if c.is_digit(10) {
                if first_number.is_empty() {
                    first_number = c.to_string();
                    second_number = c.to_string();
                } else {
                    second_number = c.to_string();
                }
            } else {
                numbers.iter().enumerate().for_each(|(d, val)| {
                    if line[i..].starts_with(val) {
                        if first_number.is_empty() {
                            first_number = (d + 1).to_string();
                            second_number = (d + 1).to_string();
                        } else {
                            second_number = (d + 1).to_string();
                        }
                    }
                });
            }
        });

        let number = format!("{}{}", first_number, second_number);

        let number = match number.parse::<i32>() {
            Ok(number) => number,
            Err(l) => {
                println!("Error: {}", l);
                0
            }
        };

        sum += number;
    });

    println!("Sum: {}", sum);
}
