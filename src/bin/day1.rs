use std::{fs::File, io::{BufReader, BufRead}, vec};


fn main() {
    let file = File::open("data/day1-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut most_cals = 0;
    // let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            sum += line.parse::<u32>().expect("Error parsing number");
        } else {
            most_cals = most_cals.max(sum);
            sum = 0;
        }
    }

    println!("{}", most_cals);
}