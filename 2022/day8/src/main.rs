use crate::parse_forest::parse_forest;
use std::fs;

mod forest;
mod parse_forest;

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();

    let (_, forest) = parse_forest(input_text.as_str()).unwrap();

    println!("{} tres are visible from the outside", forest.visible_count());

    println!("maximum scenic score is {}", forest.max_scenic_score());
}
