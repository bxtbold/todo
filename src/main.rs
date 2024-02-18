#![allow(warnings)]

mod cli;
mod utils;

use cli::Cli;
use utils::*;


fn main() {
    let cli = Cli::parse()
        .expect("Failed to parse command line arguments");
    let file_path = "/history/20240121.csv";
    match cli.execute(file_path) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
