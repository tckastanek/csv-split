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

    let splits = CsvSplits::new(&file_string, split_at_lines, prefix);
    
    for split in splits {
        let (file_data_string, file_name_string) = split;
        let new_path = file_path.with_file_name(file_name_string);
        fs::write(new_path, file_data_string).unwrap();
    }
}
