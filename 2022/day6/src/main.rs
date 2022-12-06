extern crate itertools;

use io::Read;
use std::{fs, io};
use crate::start_bytes_window::{StartOfPacket, StartOfMessage};

mod start_bytes_window;

fn main() {
    let input_file = fs::File::open("input.txt").unwrap();
    let input_reader = io::BufReader::new(input_file);

    let mut window = StartOfPacket::new();

    let mut it = input_reader
        .bytes()
        .map(|b| b.unwrap())
        .enumerate();

    let result = it
        .find(|(_, b)| {
            window.push(*b);
            window.is_starting_sequence()
        });

    if let Some((index, _)) = result {
        println!("Start of packet appeared after processing {} chars", index + 1);
    } else {
        println!("No start of packet found");
    }

    let mut window = StartOfMessage::new();

    let result = it
        .find(|(_, b)| {
            window.push(*b);
            window.is_starting_sequence()
        });

    if let Some((index, _)) = result {
        println!("Start of message appeared after processing {} chars", index + 1);
    } else {
        println!("No start of message found");
    }
}
