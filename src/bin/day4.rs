use std::{io::{BufReader, BufRead}, fs::File};
use text_io::scan;

fn main() {
    let file = File::open("data/day4-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let a1: u32;
        let a2: u32;
        let b1: u32;
        let b2: u32;
        scan!(line.bytes() => "{}-{},{}-{}", a1, a2, b1, b2);

        if a1 <= b1 && a2 >= b2 {
            sum += 1;
        } else if b1 <= a1 && b2 >= a2 {
            sum += 1;
        }
    }

    println!("{}", sum);
}