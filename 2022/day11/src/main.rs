extern crate nom;

mod monkey;
mod parse_monkey;
mod top2;

use std::fs;

fn main() {
    let input_text = fs::read_to_string("input.txt").unwrap();

    let mut troop = parse_monkey::parse_monkeys(input_text.as_str());
    let mut troop2 = troop.clone();

    for _ in 0..20 {
        troop.round(monkey::WorryDecEnabled::True);
    }

    let top2_inspectors = troop.top2_inspectors();

    println!(
        "1st run: Product of inspected count for top2 is {}",
        top2_inspectors.0.unwrap().get() * top2_inspectors.1.unwrap().get(),
    );

    for _ in 0..10_000 {
        troop2.round(monkey::WorryDecEnabled::False);
    }

    let top2_inspectors = troop2.top2_inspectors();

    println!(
        "2nd run: Product of inspected count for top2 is {}",
        top2_inspectors.0.unwrap().get() * top2_inspectors.1.unwrap().get(),
    );
}
