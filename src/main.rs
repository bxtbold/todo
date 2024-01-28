#![allow(warnings)]

mod cli;
mod utils;

use clap::Parser;
use cli::Cli;
use utils::*;


fn main() {
    let cli = Cli::parse()
        .expect("Failed to parse command line arguments");
    println!("{:?}", cli);
}
