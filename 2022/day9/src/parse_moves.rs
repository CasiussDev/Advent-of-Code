use nom::IResult;
use nom::character::complete::*;
use nom::branch::alt;
use nom::sequence::separated_pair;
use nom::combinator::map;
use nom::multi::separated_list1;
use crate::rope;

fn parse_steps(input: &str) -> IResult<&str, u8> {
    u8(input)
} 

fn parse_dir(input: &str) -> IResult<&str, rope::Direction> {
    let parser = alt((char('U'), char('D'), char('L'), char('R')));
    let mut parser = map(parser, |ch| rope::Direction::try_from(ch).unwrap());
    parser(input)
}

fn parse_move(input: &str) -> IResult<&str, rope::Move> {
    let parser = separated_pair(parse_dir, space0, parse_steps);
    let mut parser = map(parser, |(dir, steps)| rope::Move::new(dir, steps));
    parser(input)
}

pub fn parse_moves(input: &str) -> Vec<rope::Move> {
    separated_list1(newline, parse_move)(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_moves_test() {
        let input_text = fs::read_to_string("input_test.txt").unwrap();

        let v = parse_moves(input_text.as_str());

        let expected_result = vec![
          rope::Move::new(
              rope::Direction::Right,
              4
          )  ,
            rope::Move::new(
              rope::Direction::Up,
              4
          )  ,
            rope::Move::new(
              rope::Direction::Left,
              3
          )  ,
            rope::Move::new(
              rope::Direction::Down,
              1
          )  ,
            rope::Move::new(
              rope::Direction::Right,
              4
          )  ,
            rope::Move::new(
              rope::Direction::Down,
              1
          )  ,
            rope::Move::new(
              rope::Direction::Left,
              5
          )  ,
            rope::Move::new(
              rope::Direction::Right,
              2
          )  ,
            rope::Move::new(
              rope::Direction::Left,
              255
          )  ,
        ];
    }
}