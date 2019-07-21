extern crate clap;

pub mod constants;
pub mod cli;

use cli::{create_matches};
use constants::{FILE_ARG};

fn main() {
    let matches = create_matches();
    let file_path = matches.value_of(FILE_ARG).unwrap();
    dbg!(file_path);
}
