extern crate nom;

mod parse_moves;
mod parse_stacks;
mod stacks;

use stacks::Stacks;
use std::fs;

fn collect_results(stacks: &Stacks) -> String {
    stacks
        .stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();

    let (remaining, (crate_stack_rows, stack_indices)) =
        parse_stacks::parse_stack_setup(&input_text).unwrap();

    let mut stack_count = 0usize;
    assert_ne!(stack_indices.len(), 0);
    for (i, stack_index) in stack_indices.into_iter().enumerate() {
        assert_eq!(i + 1, stack_index);
        stack_count = stack_index;
    }

    let mut stacks = stacks::Stacks::new(stack_count);
    stacks.push_crates(crate_stack_rows);
    let stacks_clone = stacks.clone();

    let (_, moves) = parse_moves::parse_moves(remaining).unwrap();

    moves.clone().into_iter().for_each(|m| {
        stacks.apply_move_9000(m);
    });

    let top_stacks = collect_results(&stacks);

    println!("Top crates with CrateMover 9000 {top_stacks}");

    let mut stacks = stacks_clone;

    moves.into_iter().for_each(|m| {
        stacks.apply_move_9001(m);
    });

    let top_stacks = collect_results(&stacks);

    println!("Top crates with CrateMover 9001 {top_stacks}");
}
