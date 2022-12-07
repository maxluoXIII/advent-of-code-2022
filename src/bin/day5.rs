use std::{fs::File, io::{BufReader, BufRead}};
use sscanf::sscanf;
use itertools::Itertools;

fn move_crates(stacks: &mut Vec<Vec<char>>, dest: usize, source: usize, num: u32) {
    let mut moving = Vec::new();
    for _ in 0..num {
        moving.push(stacks[source].pop().expect("Tried to move from empty stack"));
    }
    stacks[dest].extend(moving.iter().rev());
}

fn main() {
    let file = File::open("data/day5-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut reading_stacks = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            reading_stacks = false;
            for rev_stack in &mut stacks {
                rev_stack.reverse();
            }
            continue;
        }

        if reading_stacks {
            for (stack_index, stack_area) in line.chars().chunks(4).into_iter().enumerate() {
                let stack_area: String = stack_area.collect();
                if let Ok(crate_name) = sscanf!(stack_area.trim_end(), "[{char}]") {
                    while stacks.len() <= stack_index {
                        stacks.push(Vec::new());
                    }
                    stacks[stack_index].push(crate_name);
                }
            }
        } else {
            let (num, source, dest) = sscanf!(line, "move {u32} from {usize} to {usize}").unwrap();
            move_crates(&mut stacks, dest-1, source-1, num);
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().expect("Nothing on to of stack"));
    }
}