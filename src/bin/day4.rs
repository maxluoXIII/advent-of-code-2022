use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};
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

        let assignments1: HashSet<u32> = HashSet::from_iter(a1..=a2);
        let assignments2: HashSet<u32> = HashSet::from_iter(b1..=b2);
        if assignments1.intersection(&assignments2).count() != 0 {
            sum += 1;
        }
    }

    println!("{}", sum);
}