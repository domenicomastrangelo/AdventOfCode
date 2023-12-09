use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Game {
    id: u16,
    possible: bool,
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

    let mut ids_sum: u16 = 0;

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("There was a problem reading the line: {:?}", error);
                continue;
            }
        };

        let game = line.split(":").collect::<Vec<&str>>();
        let game_id_part = game[0];
        let game_id_part = game_id_part.split(" ").collect::<Vec<&str>>();
        let game_id = match game_id_part[1].parse::<u16>() {
            Ok(game_id) => game_id,
            Err(error) => {
                println!("There was a problem parsing the game id: {:?}", error);
                continue;
            }
        };

        let plays = game[1];
        let plays = plays.split(";").collect::<Vec<&str>>();

        let mut game = Game {
            id: game_id,
            possible: true,
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
                        if n > 12 {
                            game.possible = game.possible & false;
                        }
                    }
                    "green" => {
                        if n > 13 {
                            game.possible = game.possible & false;
                        }
                    }
                    "blue" => {
                        if n > 14 {
                            game.possible = game.possible & false;
                        }
                    }
                    _ => {
                        println!("There was a problem parsing the color");
                        continue;
                    }
                };
            }
        }

        if game.possible {
            ids_sum += game.id;
        }
    }

    println!("The sum of the ids is: {}", ids_sum);
}
