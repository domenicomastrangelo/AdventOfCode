use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Colors {
    red: u16,
    green: u16,
    blue: u16,
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

    let mut power_sum: u32 = 0;

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("There was a problem reading the line: {:?}", error);
                continue;
            }
        };

        let game = line.split(":").collect::<Vec<&str>>();

        let plays = game[1];
        let plays = plays.split(";").collect::<Vec<&str>>();

        let mut game_result: u32 = 0;

        let mut colors = Colors {
            red: 0,
            green: 0,
            blue: 0,
        };

        for play in plays {
            let play = play.trim();
            let re = match Regex::new(r"([0-9]+)\s(red|green|blue),?\s?;?") {
                Ok(re) => re,
                Err(error) => {
                    println!("There was a problem creating the regex: {:?}", error);
                    continue;
                }
            };

            for match_ in re.captures_iter(play) {
                let n = match match_[1].parse::<u16>() {
                    Ok(id) => id,
                    Err(error) => {
                        println!("There was a problem parsing the id: {:?}", error);
                        continue;
                    }
                };

                let _color = match &match_[2] {
                    "red" => {
                        if colors.red < n {
                            colors.red = n;
                        }
                    }
                    "green" => {
                        if colors.green < n {
                            colors.green = n;
                        }
                    }
                    "blue" => {
                        if colors.blue < n {
                            colors.blue = n;
                        }
                    }
                    _ => {
                        println!("There was a problem parsing the color");
                        continue;
                    }
                };

                game_result = u32::from(colors.red * colors.green * colors.blue);
            }
        }

        power_sum += game_result;
    }

    println!("The sum of powers is: {}", power_sum);
}
