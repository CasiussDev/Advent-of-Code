use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

fn parse_crate(input: &str) -> IResult<&str, char> {
    let parser = tuple((tag("["), anychar, tag("]")));
    let mut parser = map(parser, |(_, ch, _)| ch);
    parser(input)
}

fn parse_empty_crate_space(input: &str) -> IResult<&str, &str> {
    tag("   ")(input)
}

fn parse_crate_space(input: &str) -> IResult<&str, Option<char>> {
    let crate_parser = map(parse_crate, |ch| Some(ch));
    let empty_space_parser = map(parse_empty_crate_space, |_| None);
    let mut parser = alt((crate_parser, empty_space_parser));
    parser(input)
}

type StacksRow = Vec<Option<char>>;

fn parse_crate_spaces_line(input: &str) -> IResult<&str, StacksRow> {
    separated_list1(char(' '), parse_crate_space)(input)
}

type StacksRows = Vec<Vec<Option<char>>>;

fn parse_crate_spaces_lines(input: &str) -> IResult<&str, StacksRows> {
    terminated(
        separated_list1(char('\n'), parse_crate_spaces_line),
        char('\n'),
    )(input)
}

type StacksIndices = Vec<usize>;

fn parse_stack_indices(input: &str) -> IResult<&str, StacksIndices> {
    separated_list1(tag("   "), map(digit1, |s| str::parse(s).unwrap()))(input)
}

fn parse_stack_indices_line(input: &str) -> IResult<&str, StacksIndices> {
    delimited(char(' '), parse_stack_indices, tag(" \n"))(input)
}

pub fn parse_stack_setup(input: &str) -> IResult<&str, (StacksRows, StacksIndices)> {
    pair(parse_crate_spaces_lines, parse_stack_indices_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    pub fn parse_stack() {
        let input_text = fs::read_to_string("test_input.txt").unwrap();

        let parse_result = parse_stack_setup(&input_text).unwrap();

        let expected_stack_setup = vec![
            vec![None, Some('D'), None],
            vec![Some('N'), Some('C'), None],
            vec![Some('Z'), Some('M'), Some('P')],
        ];

        let expected_stack_indices = vec![1usize, 2, 3];

        assert_eq!(expected_stack_setup, parse_result.1 .0);
        assert_eq!(expected_stack_indices, parse_result.1 .1);
    }
}
