use crate::stacks::Move;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn parse_move(input: &str) -> IResult<&str, Move> {
    let parser = tuple((
        tag("move "),
        digit1,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ));
    let mut parser = map(parser, |(_, crates_count, _, from, _, to)| Move {
        from: str::parse::<usize>(from).unwrap() - 1,
        to: str::parse::<usize>(to).unwrap() - 1,
        crates_count: str::parse(crates_count).unwrap(),
    });
    parser(input)
}

pub fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let parser = separated_list1(char('\n'), parse_move);
    let mut parser = preceded(opt(char('\n')), parser);

    parser(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_stacks::parse_stack_setup;
    use std::fs;

    #[test]
    fn parse_moves() {
        let input_text = fs::read_to_string("test_input.txt").unwrap();

        let (remaining, _) = parse_stack_setup(&input_text).unwrap();

        let (_, moves) = super::parse_moves(remaining).unwrap();

        let expected_results = vec![
            Move {
                crates_count: 1,
                from: 2,
                to: 1,
            },
            Move {
                crates_count: 3,
                from: 1,
                to: 3,
            },
            Move {
                crates_count: 2,
                from: 2,
                to: 1,
            },
            Move {
                crates_count: 1,
                from: 1,
                to: 2,
            },
        ];

        assert_eq!(moves, expected_results);
    }
}
