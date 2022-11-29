extern crate itertools;

use itertools::Itertools;
use std::io;
use std::io::BufRead;
use std::fs;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let file_reader = io::BufReader::new(file);

    let increase_count = file_reader
        .lines()
        .filter_map(|line|line.ok())
        .filter_map(|line| line.parse::<i64>().ok())
        .tuple_windows()
        .filter(|(lhs, rhs)| lhs < rhs)
        .count();
    println!("{increase_count}");

    Ok(())
}
