use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;
use simple_error::SimpleError;

enum RpsChoice {
    Rock,
    Paper,
    Scissors
}

fn rps_value(play: RpsChoice, opponent: RpsChoice) -> u32 {
    match play {
        RpsChoice::Rock => {
            1 + match opponent {
                RpsChoice::Rock => 3,
                RpsChoice::Paper => 0,
                RpsChoice::Scissors => 6
            }
        },
        RpsChoice::Paper => {
            2 + match opponent {
                RpsChoice::Rock => 6,
                RpsChoice::Paper => 3,
                RpsChoice::Scissors => 0
            }
        },
        RpsChoice::Scissors=> {
            3 + match opponent {
                RpsChoice::Rock => 0,
                RpsChoice::Paper => 6,
                RpsChoice::Scissors => 3
            }
        }
    }
}

fn main() {
    let file = File::open("data/day2-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut score_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some((opponent, mine)) = line.split_ascii_whitespace().collect_tuple() {
            let opponent_choice = match opponent {
                "A" => Ok(RpsChoice::Rock),
                "B" => Ok(RpsChoice::Paper),
                "C" => Ok(RpsChoice::Scissors),
                _ => Err(SimpleError::new("Could not parse RPS choice"))
            }.unwrap();
            let my_choice = match mine {
                "X" => Ok(RpsChoice::Rock),
                "Y" => Ok(RpsChoice::Paper),
                "Z" => Ok(RpsChoice::Scissors),
                _ => Err(SimpleError::new("Could not parse RPS choice"))
            }.unwrap();

            score_sum += rps_value(my_choice, opponent_choice);
        }
    }

    println!("{}", score_sum);
}