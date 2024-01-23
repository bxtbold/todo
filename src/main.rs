// use clap::{App, Arg};
mod cli;
mod utils;

use cli::Cli;
use utils::*;

fn main() {
    let path = "/history/20240121.csv";
    let cli = Cli::new();
    cli.read_today(path);
}
