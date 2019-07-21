extern crate clap;
use clap::{Arg, App, ArgMatches, crate_version, crate_name, crate_authors};
use crate::constants::{FILE_ARG};

pub fn create_matches() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple utility to split CSV files, while maintaining their headers.")
        .arg(Arg::with_name(FILE_ARG)
            .help("The file to split.")
            .required(true))
        .get_matches()
}