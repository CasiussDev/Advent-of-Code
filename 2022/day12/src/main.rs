
mod parse_heightmap;
mod heightmap;

use std::fs;

fn find_remaining_path(heightmap: &heightmap::Heightmap, path: heightmap::Path, dir: heightmap::Direction) -> bool {
    let new_step = path.new_step()
}

fn find_best_path(heightmap: &heightmap::Heightmap) -> heightmap::Path {
    let path = heightmap::new_path();
}

fn main() {
    println!("Hello, world!");

    let input_text = fs::read_to_string("test_input.txt").unwrap();

    let heightmap = parse_heightmap::parse_heightmap(input_text.as_str());

    //println!("{heightmap:#?}");

    let best_path = find_best_path(&heightmap);

    println!("Best path visits {} positions", best_path.len());
}
