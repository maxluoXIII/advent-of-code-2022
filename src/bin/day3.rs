use std::{io::{BufReader, BufRead}, fs::File};


fn main() {
    let file = File::open("data/day3-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut group = Vec::new();
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        group.push(line);

        if group.len() == 3 {
            for item in group[0].chars() {
                if group[1].contains(item) && group[2].contains(item) {
                    sum += if item.is_lowercase() {
                        item as u32 - 'a' as u32 + 1
                    } else {
                        item as u32 -'A' as u32 + 27
                    };
                    break
                }
            }
            group.clear();
        }
    }

    println!("{}", sum);
}