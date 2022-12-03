extern crate itertools;

use crate::itertools::Itertools;
use std::fs;

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!("{c} is not an ASCII letter");
    }
}

fn main() {
    let file_text = fs::read_to_string("input.txt").unwrap();

    let prio_sum: u32 = file_text
        .split("\n")
        .filter(|s| s != &"")
        .map(|line| {
            let middle = line.len() / 2;
            (&line[..middle], &line[middle..])
        })
        .map(|(left, right)| left.chars().find(|ch| right.find(*ch).is_some()))
        .map(|ch| priority(ch.unwrap()))
        .sum();
    println!("Sum of priorities of items in both pockets is: {prio_sum}");

    let chunks = file_text
        .split("\n")
        .filter(|s| s != &"")
        .chunks(3);

    let prio_sum: u32 = chunks
        .into_iter()
        .map(|mut chunk| {
            let first = chunk.next().unwrap();
            let second = chunk.next().unwrap();
            let third = chunk.next().unwrap();

            first.chars().find( |ch| {
                second.find(*ch).is_some() && third.find(*ch).is_some()
            })
        })
        .map(|ch| priority(ch.unwrap()))
        .sum();

    println!("Sum of priorities of each 3-elf team item is: {prio_sum}");
}
