extern crate nom;

use nom::{
    character::complete::{newline, none_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use crate::forest::Forest;

fn parse_tree(input: &str) -> IResult<&str, u8> {
    let mut parser = map(none_of("\n"), |ch| ch.to_digit(10).unwrap() as u8);
    parser(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<u8>> {
    many1(parse_tree)(input)
}

pub fn parse_forest(input: &str) -> IResult<&str, Forest> {
    let parser = separated_list1(newline, parse_row);
    let mut parser = map(parser, |rows| {
        let mut forest = Forest::new();
        rows.into_iter().for_each(|row| forest.add_row(row));
        forest
    });
    parser(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_forest_test() {
        let input_text = fs::read_to_string("test_input.txt").unwrap();

        let (_, forest) = parse_forest(input_text.as_str()).unwrap();

        let mut expected_result = Forest::new();

        let v = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        v.into_iter().for_each(|row| expected_result.add_row(row));

        assert_eq!(expected_result, forest);
    }
}
