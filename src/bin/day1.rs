use std::{fs::File, io::{BufReader, BufRead}, collections::BinaryHeap};


fn main() {
    let file = File::open("data/day1-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut heap = BinaryHeap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line != "" {
            sum += line.parse::<u32>().expect("Error parsing number");
        } else {
            heap.push(sum);
            sum = 0;
        }
    }

    let mut top_3 = 0;
    for _ in 0..3 {
        top_3 += heap.pop().unwrap_or_default();
    }

    println!("{}", top_3);
}