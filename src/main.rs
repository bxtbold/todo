// use clap::{App, Arg};
mod io;
mod format;
mod cli;
mod task;

use cli::Cli;


fn main() {
    let path = "/home/bat/ws/rust_ws/todo/lists/20240121.csv";
    let cli = Cli::new();
    cli.read_today(path);
}
