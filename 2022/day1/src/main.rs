extern crate itertools;

mod fold_multiple;
mod top3;

use crate::top3::Top3;
use fold_multiple::FoldMultiple;
use itertools::FoldWhile::{Continue, Done};
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("input.txt").unwrap();
    let file_reader = io::BufReader::new(file);

    let results = file_reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u64>().ok())
        .fold_multiple(0u64, |acc, x| {
            if let Some(cal) = x {
                Continue(acc + cal)
            } else {
                Done(acc)
            }
        })
        .top3();
    println!(
        "The elf with more calories carries {} cal",
        results.0.unwrap()
    );
    println!(
        "The top 3 elves with more calories carry a total of {} cal",
        results.0.unwrap() + results.1.unwrap() + results.2.unwrap()
    );
}
