use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Hash, PartialEq, Eq, Clone)]
struct Pos(i32, i32);

fn main() {
    let file = File::open("data/day9-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut head_pos = Pos(0, 0);
    let mut tail_pos = Pos(0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail_pos.clone());

    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, mag) =
            sscanf::sscanf!(line, "{char} {usize}").expect("Line does not match expected format");
        
        for _ in 0..mag {
            match dir {
                'R' => {
                    head_pos.0 += 1;
                },
                'D' => {
                    head_pos.1 -= 1;
                },
                'L' => {
                    head_pos.0 -= 1;
                },
                'U' => {
                    head_pos.1 += 1;
                },
                _ => {
                    panic!("Unexpected direction");
                }
            }

            let x_diff = head_pos.0 - tail_pos.0;
            let y_diff = head_pos.1 - tail_pos.1;
            if x_diff.abs() == 2 {
                tail_pos.0 += x_diff / 2;
                tail_pos.1 += y_diff;
            } else if y_diff.abs() == 2 {
                tail_pos.1 += y_diff / 2;
                tail_pos.0 += x_diff;
            }

            visited.insert(tail_pos.clone());
        }
    }

    println!("Tail visited {}", visited.len());
}
