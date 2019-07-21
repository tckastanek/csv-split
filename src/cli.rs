extern crate clap;
use crate::constants::{FILE_ARG, LINES_ARG};
use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches};

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
            Arg::with_name(LINES_ARG)
                .help("Set number of lines to split at.")
                .short("l")
                .takes_value(true)
                .default_value("1000"),
        )
        .get_matches()
}
