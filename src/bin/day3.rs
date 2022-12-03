use std::{io::{BufReader, BufRead}, fs::File};


fn main() {
    let file = File::open("data/day3-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let first_compartment = &line[..line.len()/2];
        let second_compartment = &line[line.len()/2..];

        for item in first_compartment.chars() {
            if second_compartment.contains(item) {
                sum += if item.is_lowercase() {
                    item as u32 - 'a' as u32 + 1
                } else {
                    item as u32 -'A' as u32 + 27
                };
                break
            }
        }
    }

    println!("{}", sum);
}