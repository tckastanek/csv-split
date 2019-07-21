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
    let (file_path, split_at_lines) = parse_matches(&matches);

    let file_string = fs::read_to_string(file_path).unwrap();

    let mut splits = CsvSplits::new(&file_string, split_at_lines);

    //    let first_split = splits.next().unwrap();
    //    dbg!(first_split.lines().collect::<Vec<&str>>());
    //
    //    let second_split = splits.next().unwrap();
    //    dbg!(second_split.lines().collect::<Vec<&str>>());
}
