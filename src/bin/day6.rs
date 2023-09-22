use std::{
    collections::{HashSet},
    fs::File,
    io::{BufReader, Read},
};

const START_MARKER_LENGTH: usize = 14;

fn get_start_marker_pos(datastream: Vec<u8>) -> Option<usize> {
    let mut curr = datastream.iter();
    let mut pos = 0;
    loop {
        let start = curr.clone();
        let set = HashSet::<&u8>::from_iter(start.take(START_MARKER_LENGTH));
        if set.len() == START_MARKER_LENGTH {
            return Some(pos + START_MARKER_LENGTH);
        }

        if curr.next() == None {
            break;
        }
        pos += 1;
    }
    return None;
}

fn main() {
    let file = File::open("data/day6-full.txt").expect("Could not find data file");
    let mut reader = BufReader::new(file);
    let mut datastream = Vec::new();
    reader.read_to_end(&mut datastream).expect("Error reading file");
    let marker_pos = get_start_marker_pos(datastream);
    println!("Start marker position: {:?}", marker_pos);
}
