extern crate clap;

pub mod cli;
pub mod constants;
pub mod splits;

use cli::{create_matches, parse_matches};
use splits::CsvSplits;
use std::fs;

// TODO: Could be fun to have separate space and time complexity optimized paths depending on workload
fn main() {
    let matches = create_matches();
    let (file_path, split_at_lines, prefix) = parse_matches(&matches);
    
    // TODO: Maybe try a BufReader to see if that's less memory intensive than reading the whole file at once
    let file_string = fs::read_to_string(file_path).unwrap();

    let mut splits = CsvSplits::new(&file_string, split_at_lines, prefix);

//    let first_split = splits.next().unwrap();
//    dbg!(first_split.0.lines().collect::<Vec<&str>>());
//    dbg!(first_split.1);
//
//    let second_split = splits.next().unwrap();
//    dbg!(second_split.0.lines().collect::<Vec<&str>>());
//    dbg!(second_split.1);
}
