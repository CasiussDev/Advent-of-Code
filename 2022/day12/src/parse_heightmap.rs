extern crate nom;

use nom::{
    character::complete::{newline, none_of, satisfy, char},
    combinator::map,
    multi::{many1, separated_list1},
    branch::alt,
    IResult,
};

use crate::heightmap;

fn parse_start(input: &str) -> IResult<&str, u8>{
    let parser = char('S');
    let mut parser = map(parser, |_| heightmap::START_POS_KEY);
    parser(input)
}

fn parse_end(input: &str) -> IResult<&str, u8> {
    let parser = char('E');
    let mut parser = map(parser, |_| heightmap::END_POS_KEY);
    parser(input)    
}

pub fn parse_pos(input: &str) -> IResult<&str, u8> {
    alt((parse_start, parse_end, parse_height))(input)
}

fn parse_height(input: &str) -> IResult<&str, u8> {
    let parser = satisfy(|ch| (ch >= 'a') && (ch <= 'z'));
    let mut parser = map(parser, |ch| (ch as u8) - b'a');
    parser(input)
}

fn parse_heightrow(input: &str) -> IResult<&str, Vec<u8>> {
    many1(parse_pos)(input)
}

pub fn parse_heightmap(input: &str) -> heightmap::Heightmap {
    let mut parser = separated_list1(newline, parse_heightrow);
    let mut elevations = parser(input).unwrap().1;
    heightmap::Heightmap::new(elevations)
}