use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, opt},
    multi::{many0, many1_count, separated_list0},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::commands::*;
use crate::file_system::*;

fn parse_destination(input: &str) -> IResult<&str, DirectoryDestination> {
    let root_parser = map(char('/'), |_| DirectoryDestination::Root);
    let parent_parser = map(tag(".."), |_| DirectoryDestination::Parent);
    let child_parser = map(alpha1, |dir| DirectoryDestination::Child(dir));

    let mut parser = alt((root_parser, parent_parser, child_parser));
    parser(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let parser = preceded(tag("cd "), parse_destination);
    let mut parser = map(parser, |dst| Command::ChangeDirectory(dst));
    parser(input)
}

fn parse_file(input: &str) -> IResult<&str, Child> {
    let parser = many1_count(alt((alpha1, tag("."))));
    let parser = separated_pair(digit1, char(' '), parser);
    let mut parser = map(parser, |(size, _)| {
        Child::File(str::parse::<FileSizeInt>(size).unwrap())
    });
    parser(input)
}

fn parse_dir(input: &str) -> IResult<&str, Child> {
    let parser = separated_pair(tag("dir"), char(' '), alpha1);
    let mut parser = map(parser, |(_, name)| Child::Directory { name, id: None });
    parser(input)
}

fn parse_ls_results(input: &str) -> IResult<&str, ChildList> {
    let parser = alt((parse_dir, parse_file));
    let mut parser = separated_list0(char('\n'), parser);
    parser(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let parser = preceded(tag("ls\n"), parse_ls_results);
    let mut parser = map(parser, |v| Command::List(v));
    parser(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let parser = alt((parse_ls, parse_cd));
    let mut parser = delimited(tag("$ "), parser, opt(char('\n')));
    parser(input)
}

pub fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    many0(parse_command)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example_parse_commands() {
        let input_text = fs::read_to_string("test_input.txt").unwrap();

        let commands = parse_commands(&input_text[..]).unwrap();

        let expected_result = vec![
            Command::ChangeDirectory(DirectoryDestination::Root),
            Command::List(vec![0, 14848514, 8504156, 0]),
            Command::ChangeDirectory(DirectoryDestination::Child("a")),
            Command::List(vec![0, 29116, 2557, 62596]),
        ];

        assert_eq!(expected_result, commands.1);
    }
}
