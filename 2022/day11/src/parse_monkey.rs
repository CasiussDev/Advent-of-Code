use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::map;
use nom::multi::{separated_list1, separated_list0};
use nom::sequence::{delimited, preceded, pair, tuple};
use nom::IResult;

use crate::monkey;

fn parse_id(input: &str) -> IResult<&str, ()> {
    let parser = tuple((tag("Monkey "), digit1, char(':'), newline));
    let mut parser = map(parser, |_| ());
    parser(input)
}

fn parse_item(input: &str) -> IResult<&str, monkey::WorryLevel> {
    map(u128, |id| monkey::WorryLevel::new(id))(input)
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<monkey::WorryLevel>> {
    let prefix = pair(space1, tag("Starting items: "));
    let parser = separated_list0(tag(", "), parse_item);
    let mut parser = delimited(prefix, parser, newline);
    parser(input)
}

fn parse_square(input: &str) -> IResult<&str, monkey::WorryOp> {
    let parser = tag("new = old * old");
    let mut parser = map(parser, |_| monkey::WorryOp::Square);
    parser(input)
}

fn parse_add(input: &str) -> IResult<&str, monkey::WorryOp> {
    let parser = preceded(tag("new = old + "), u16);
    let mut parser = map(parser, |sumand| monkey::WorryOp::Add(sumand));
    parser(input)
}

fn parse_mul(input: &str) -> IResult<&str, monkey::WorryOp> {
    let parser = preceded(tag("new = old * "), u16);
    let mut parser = map(parser, |factor| monkey::WorryOp::Mul(factor));
    parser(input)
}

fn parse_operation(input: &str) -> IResult<&str, monkey::WorryOp> {
    let parser = alt((parse_square, parse_add, parse_mul));
    let mut parser = delimited(tuple((space1, tag("Operation: "))), parser, newline);
    parser(input)
}

fn parse_test(input: &str) -> IResult<&str, monkey::DivisibleBy> {
    let prefix = pair(space1, tag("Test: divisible by "));
    let parser = delimited(prefix, u16, newline);
    let mut parser = map(parser, |d| monkey::DivisibleBy::new(d));
    parser(input)
}

fn parse_if<'a>(input: &'a str, cond: &str) -> IResult<&'a str, monkey::MonkeyId> {
    let prefix = pair(space1, tag(cond));
    let parser = preceded(prefix, u32);
    let mut parser = map(parser, |id| monkey::MonkeyId::new(id as usize));
    parser(input)
}

fn parse_if_true(input: &str) -> IResult<&str, monkey::MonkeyId> {
    parse_if(input, "If true: throw to monkey ")
}

fn parse_if_false(input: &str) -> IResult<&str, monkey::MonkeyId> {
    parse_if(input, "If false: throw to monkey ")
}

fn parse_monkey(input: &str) -> IResult<&str, monkey::Monkey> {
    let parser = tuple((
        parse_id,
        parse_starting_items,
        parse_operation,
        parse_test,
        parse_if_true,
        newline,
        parse_if_false,
    ));
    let mut parser = map(parser, |(_, items, op, test, if_true, _, if_false)| {
        monkey::Monkey::new(&items, op, test, if_true, if_false)
    });

    parser(input)
}

pub fn parse_monkeys(input: &str) -> monkey::Troop {
    let monkeys = separated_list1(pair(newline, newline), parse_monkey)(input).unwrap().1;
    monkey::Troop::new(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_monkeys_test(){
        let input_text = fs::read_to_string("test_input.txt").unwrap();

        let result = parse_monkeys(input_text.as_str());

        println!("{result:?}");
    }
}