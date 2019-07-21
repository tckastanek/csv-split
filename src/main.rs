extern crate clap;

pub mod cli;
pub mod constants;

use clap::ArgMatches;
use cli::create_matches;
use constants::{CSV_EXTENSION, FILE_ARG, LINES_ARG};
use std::{fs, path::Path};

// TODO: ERRORS!@@@
fn get_file_path(file_path_str: &str) -> &Path {
    let file_path = Path::new(file_path_str);
    let ext = file_path.extension().unwrap();
    if ext != CSV_EXTENSION {
        panic!()
    } else {
        file_path
    }
}

fn parse_matches<'a>(matches: &'a ArgMatches) -> (&'a str, usize) {
    let file_path = matches.value_of(FILE_ARG).unwrap(); // is required arg
    let split_at_lines: usize = matches.value_of(LINES_ARG).unwrap().parse().unwrap();
    (file_path, split_at_lines)
}

fn split_lines(file_string: String) {
    let lines: Vec<&str> = file_string.lines().collect();
    let num_lines = lines.len();
    dbg!(num_lines);
}

// TODO: Could be fun to have separate space and time complexity optimized paths depending on workload
fn main() {
    let matches = create_matches();
    let (file_path_str, split_at_lines) = parse_matches(&matches);
    let file_path = get_file_path(file_path_str);

    let file_string = fs::read_to_string(file_path).unwrap();
    let lines = split_lines(file_string);
}
