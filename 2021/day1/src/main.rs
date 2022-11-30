extern crate itertools;

use itertools::Itertools;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("input.txt")?;
    let file_reader = io::BufReader::new(file);

    let samples = file_reader
        .lines()
        .map(|line| line.map_err(|error| Box::<dyn std::error::Error>::from(error)))
        .map(|line| {
            line.and_then(|line| {
                line.parse::<i64>()
                    .map_err(|error| Box::<dyn std::error::Error>::from(error))
            })
        })
        .collect::<Result<Vec<i64>, Box<dyn std::error::Error>>>()?;

    let increase_depth_count = samples
        .iter()
        .tuple_windows()
        .filter(|(lhs, rhs)| rhs > lhs)
        .count();
    println!("{increase_depth_count} samples are greater than the previous one");

    let increase_depth_sum_count = samples
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .filter(|(lhs, rhs)| rhs > lhs)
        .count();
    println!("{increase_depth_sum_count} 3-sample sums are greater than the previous one");

    Ok(())
}
