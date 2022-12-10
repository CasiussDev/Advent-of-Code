extern crate nom;

use crate::commands::DirectoryDestination::Parent;
use crate::commands::{Command, DirectoryDestination};
use crate::console_parser::parse_commands;
use crate::file_system::{Directories, FileSizeInt};
use std::fs;

mod commands;
mod console_parser;
mod file_system;

fn main() {
    let file_text = fs::read_to_string("input.txt").unwrap();

    let (remaining, commands) = parse_commands(file_text.as_str()).unwrap();

    assert_eq!(remaining, "");

    let mut dirs = Directories::new();

    println!("{commands:?}");

    for command in commands {
        match command {
            Command::ChangeDirectory(dst) => match dst {
                DirectoryDestination::Root => dirs.cd_root(),
                Parent => dirs.cd_parent(),
                DirectoryDestination::Child(name, ..) => dirs.cd_child(name),
            },
            Command::List(children) => {
                dirs.add_children(children);
            }
        }
    }

    let sum: FileSizeInt = dirs
        .dirs()
        .filter_map(|dir| {
            if dir.size <= 100000 {
                return Some(dir.size)
            }
            None
        })
        .sum();
    println!("Directories with size less than 100000 add up to {sum}");

    const NEEDED_SPACE: usize = 30000000;
    const TOTAL_SPACE: usize = 70000000;

    let used_size = dirs.dirs().next().unwrap().size;

    let space_to_free = NEEDED_SPACE - (TOTAL_SPACE - used_size);

    let dir_size = dirs.dirs().filter(|dir| {
        dir.size >= space_to_free
    }).min_by_key(|dir| dir.size).unwrap().size;

    println!("The smallest directory that frees up enough space has a size of {dir_size}");
}
