extern crate nom;

mod rope;
mod parse_moves;

use std::fs;
use parse_moves::parse_moves;

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();
    let moves = parse_moves(input_text.as_str());

    let mut rope = rope::Rope::<2>::new();
    rope.apply_moves(moves.as_slice());
    println!("First rope tail visits {} positions", rope.visited_by_tail_count());

    let mut rope = rope::Rope::<10>::new();
    rope.apply_moves(moves.as_slice());
    println!("Second rope tail visits {} positions", rope.visited_by_tail_count());
}
