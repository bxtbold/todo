#![allow(warnings)]

mod cli;
mod utils;

use cli::Cli;
use utils::*;


fn main() {
    // file path is set to the default path if the environment variable is not set
    let default_file_path = format!("/home/{}/todo_history/{}.csv", whoami::username(), get_today_date());
    let file_path = std::env::var("TODO_HISTORY_PATH")
        .unwrap_or(default_file_path);

    // check if the file exists, if not create it
    check_today_file(&file_path);

    // parse the command line arguments and execute the command
    let cli = Cli::parse()
        .expect("Failed to parse command line arguments");
    match cli.execute(&file_path) {
       Ok(_) => (),
       Err(e) => println!("{}", e),
    }
}
