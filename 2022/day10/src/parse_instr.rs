use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::device;

fn parse_noop(input: &str) -> IResult<&str, device::Instr> {
    map(tag("noop"), |_| device::Instr::NoOp)(input)
}

fn parse_addx(input: &str) -> IResult<&str, device::Instr> {
    let parser = separated_pair(tag("addx"), space1, i16);
    let mut parser = map(parser, |(_, d)| device::Instr::AddX(d));
    parser(input)
}

fn parse_instruction(input: &str) -> IResult<&str, device::Instr> {
    alt((parse_noop, parse_addx))(input)
}

pub fn parse_instructions(input: &str) -> Vec<device::Instr> {
    separated_list1(newline, parse_instruction)(input)
        .unwrap()
        .1
}
