use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Pos(i32, i32);

fn main() {
    let file = File::open("data/day9-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    const ROPE_LEN: usize = 10;
    let mut rope = [Pos::default(); ROPE_LEN];
    let mut visited = HashSet::new();
    visited.insert(rope[ROPE_LEN - 1]);

    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, mag) =
            sscanf::sscanf!(line, "{char} {usize}").expect("Line does not match expected format");

        for _ in 0..mag {
            for rope_idx in 0..ROPE_LEN {
                if rope_idx == 0 {
                    let head_pos = &mut rope[0];
                    match dir {
                        'R' => {
                            head_pos.0 += 1;
                        }
                        'D' => {
                            head_pos.1 -= 1;
                        }
                        'L' => {
                            head_pos.0 -= 1;
                        }
                        'U' => {
                            head_pos.1 += 1;
                        }
                        _ => {
                            panic!("Unexpected direction");
                        }
                    }
                } else {
                    let (prev_seg, curr_seg) = rope.split_at_mut(rope_idx);
                    let prev_seg_pos = &prev_seg[rope_idx - 1];
                    let curr_seg_pos = &mut curr_seg[0];

                    let x_diff = prev_seg_pos.0 - curr_seg_pos.0;
                    let y_diff = prev_seg_pos.1 - curr_seg_pos.1;
                    if x_diff.abs() == 2 && y_diff.abs() == 2 {
                        curr_seg_pos.0 += x_diff / 2;
                        curr_seg_pos.1 += y_diff / 2;
                    } else if x_diff.abs() == 2 {
                        curr_seg_pos.0 += x_diff / 2;
                        curr_seg_pos.1 += y_diff;
                    } else if y_diff.abs() == 2 {
                        curr_seg_pos.1 += y_diff / 2;
                        curr_seg_pos.0 += x_diff;
                    }

                    if rope_idx == ROPE_LEN - 1 {
                        visited.insert(*curr_seg_pos);
                    }
                }
            }
        }
    }

    println!("Tail visited {}", visited.len());
}
