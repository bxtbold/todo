// use clap::{App, Arg};
mod cli;
mod utils;

use cli::Cli;
use utils::*;

fn main() {
    let mut cli = Cli::new("20240121".to_string());
    cli.list();

    println!("Adding a task");

    cli.add("New task", "high", "deadline_new");
    cli.list();

    println!("Completing a task");
    cli.complete("New task");
    cli.list();

    println!("Removing a task");
    cli.remove("task1");
    cli.list();

    println!("Sorting tasks");
    cli.sort();
    cli.list();
}
