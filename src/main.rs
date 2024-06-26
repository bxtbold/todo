#![allow(warnings)]

mod cli;
mod utils;

use cli::Cli;
use utils::*;


fn main() {
    // file path is set to the default path if the environment variable is not set
    let default_path = format!("/{}/todo_history", std::env::var("HOME").unwrap());
    let path = std::env::var("TODO_HISTORY_PATH").unwrap_or(default_path);
    let file_path = format!("{}/{}.csv", path, get_today_date());

    // check if the file exists, if not create it
    check_today_file(&file_path);

    let mut task_list = TaskList::load_tasks_from_csv(&file_path).unwrap();
    // parse the command line arguments and execute the command
    let cli = Cli::parse()
        .expect("Failed to parse command line arguments");

    if cli.execute(&mut task_list).is_ok() {
        task_list.save_tasks_to_csv();
    }
}
