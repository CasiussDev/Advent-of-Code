extern crate nom;

use crate::janken::{GameRound, Move, RoundResult};
use nom::{branch::alt, character::complete::char, sequence::separated_pair, IResult};
use std::fs;

mod janken;

fn parse_line(input: &str) -> IResult<&str, (char, char)> {
    let opponent_move_parser = alt((char('A'), char('B'), char('C')));
    let own_move_parser = alt((char('X'), char('Y'), char('Z')));
    let mut parser = separated_pair(opponent_move_parser, char(' '), own_move_parser);

    parser(input)
}

fn main() {
    let file_text = fs::read_to_string("input.txt").unwrap();

    let char_rounds: Vec<_> = file_text
        .split("\n")
        .filter(|s| s != &"")
        .map(|line| parse_line(&line).unwrap().1)
        .collect();

    let final_score: u32 = char_rounds
        .iter()
        .map(|(opponent, own)| GameRound::try_from((*opponent, *own)).unwrap())
        .map(|game_round| game_round.score())
        .sum();
    println!("First calculation of final score is {final_score}");

    let final_score: u32 = char_rounds
        .iter()
        .map(|(opponent, res)| {
            (
                Move::try_from(*opponent).unwrap(),
                RoundResult::try_from(*res).unwrap(),
            )
        })
        .map(|(opponent, res)| GameRound::with_result(opponent, res))
        .map(|game_round| game_round.score())
        .sum();
    println!("Second calculation of final score is {final_score}");
}
