extern crate nom;

mod contains;
mod overlaps;

use contains::Contains;
use overlaps::Overlaps;

use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};
use std::fs;

fn parse_line(s: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    let mut parser = separated_pair(
        separated_pair(
            map_res(digit1, str::parse),
            char('-'),
            map_res(digit1, str::parse),
        ),
        char(','),
        separated_pair(
            map_res(digit1, str::parse),
            char('-'),
            map_res(digit1, str::parse),
        ),
    );

    parser(s)
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();

    let overlapped_count = input_text
        .split("\n")
        .filter(|line| line != &"")
        .map(|line| parse_line(line).unwrap().1)
        .fold((0u32, 0u32), |mut acc, (lhs, rhs)| {
            if Contains::any_contains_other(&lhs, &rhs) {
                acc.0 += 1;
            }
            if lhs.overlaps(&rhs) {
                acc.1 += 1;
            }
            acc
        });
    println!(
        "{} elves have their range contained within their partner's",
        overlapped_count.0
    );
    println!(
        "{} pairs of elves have ranges that overlap",
        overlapped_count.1
    );
}
