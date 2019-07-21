extern crate clap;
use crate::constants::{CSV_EXTENSION, FILE_ARG, LINES_ARG, PREFIX_ARG};
use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches};
use std::path::Path;

pub fn create_matches() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple utility to split CSV files, while maintaining their headers.")
        .arg(
            Arg::with_name(FILE_ARG)
                .help("The file to split.")
                .required(true),
        )
        .arg(
            Arg::with_name(PREFIX_ARG)
                .help("Name for the output files.")
                .default_value("x"),
        )
        .arg(
            Arg::with_name(LINES_ARG)
                .help("Set number of lines to split at.")
                .short("l")
                .takes_value(true)
                .default_value("1000"),
        )
        .get_matches()
}

fn get_file_path(file_path_str: &str) -> &Path {
    let file_path = Path::new(file_path_str);
    let ext = file_path.extension().unwrap();
    if ext != CSV_EXTENSION {
        panic!()
    } else {
        file_path
    }
}

pub fn parse_matches<'a>(matches: &'a ArgMatches) -> (&'a Path, usize, &'a str) {
    let file_path_str = matches.value_of(FILE_ARG).unwrap(); // is required arg
    let file_path = get_file_path(file_path_str);

    let split_at_lines: usize = matches.value_of(LINES_ARG).unwrap().parse().unwrap();

    let prefix = matches.value_of(PREFIX_ARG).unwrap(); // has default

    (file_path, split_at_lines, prefix)
}
