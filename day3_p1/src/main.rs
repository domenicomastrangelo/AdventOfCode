use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug)]
struct Number {
    value: u32,
    y_position: Option<isize>,
    x_start_position: Option<isize>,
    x_end_position: Option<isize>,
}

#[derive(Debug)]
struct Symbol {
    x_position: Option<isize>,
    y_position: Option<isize>,
}

fn main() {
    let file = File::open("input.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    let lines = BufReader::new(file).lines();

    let mut numbers: Vec<Number> = Vec::new();
    let mut tmp_num = "".to_string();

    let mut symbols: Vec<Symbol> = Vec::new();

    let mut tmp_start_position = None;

    let mut line_length = 0;

    lines.enumerate().for_each(|(line_index, line)| {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                panic!("There was a problem reading the line: {:?}", error)
            }
        };

        if line_length == 0 {
            line_length = line.len();
        }

        line.chars().enumerate().for_each(|(i, c)| {
            if c.is_digit(10) {
                tmp_num.push(c);
                if tmp_start_position == None {
                    tmp_start_position = Some(i as isize);
                }
            } else {
                if c != '.' {
                    symbols.push(Symbol {
                        x_position: Some(i as isize),
                        y_position: Some(line_index as isize),
                    });
                }

                match tmp_num.parse::<u32>() {
                    Ok(num) => {
                        let x_end_position: Option<isize>;
                        let y_position: Option<isize>;

                        if i as isize - 1 < 0 {
                            y_position = Some(line_index as isize - 1);
                            x_end_position = Some(line_length as isize - 1 as isize);
                        } else {
                            y_position = Some(line_index as isize);
                            x_end_position = Some(i as isize - 1);
                        }

                        numbers.push(Number {
                            value: num,
                            y_position,
                            x_start_position: tmp_start_position,
                            x_end_position,
                        });
                        tmp_num = "".to_string();
                        tmp_start_position = None;
                    }
                    Err(_) => {}
                }
            }
        });
    });

    let sum = get_collitions_sum(&mut numbers, &mut symbols);

    println!("Sum: {}", sum);
}

fn get_collitions_sum(numbers: &mut Vec<Number>, symbols: &mut Vec<Symbol>) -> u32 {
    let mut sum = 0;

    numbers.iter().for_each(|n| {
        let ny = n.y_position.unwrap();
        let nx_start = n.x_start_position.unwrap();
        let nx_end = n.x_end_position.unwrap();

        symbols.iter().for_each(|s| {
            let sx = s.x_position.unwrap();
            let sy = s.y_position.unwrap();

            if ny == sy && (nx_start - 1 == sx || nx_end + 1 == sx) {
                sum += n.value;
            }

            if (ny == sy + 1 || ny == sy - 1) && (sx >= nx_start - 1 && sx <= nx_end + 1) {
                sum += n.value;
            }
        });
    });

    sum
}
