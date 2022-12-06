extern crate itertools;

use crate::start_bytes_window::{StartBytesWindow, StartOfMessage, StartOfPacket};
use io::Read;
use std::{fs, io};

mod start_bytes_window;

fn find_start_sequence<I, const N: usize>(
    it: &mut I,
    mut window: StartBytesWindow<N>,
) -> Option<(usize, u8)>
where
    I: Iterator<Item = (usize, u8)>,
{
    it.find(|(_, b)| {
        window.push(*b);
        window.is_starting_sequence()
    })
}

fn main() {
    let input_file = fs::File::open("input.txt").unwrap();
    let input_reader = io::BufReader::new(input_file);

    let mut window = StartOfPacket::new();

    let mut it = input_reader.bytes().map(|b| b.unwrap()).enumerate();

    let result = find_start_sequence(&mut it, window);

    if let Some((index, _)) = result {
        println!(
            "Start of packet appeared after processing {} chars",
            index + 1
        );
    } else {
        println!("No start of packet found");
    }

    let mut window = StartOfMessage::new();

    let result = find_start_sequence(&mut it, window);

    if let Some((index, _)) = result {
        println!(
            "Start of message appeared after processing {} chars",
            index + 1
        );
    } else {
        println!("No start of message found");
    }
}
