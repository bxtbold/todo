use crate::{TaskList, strike, utils::*};
use std::io::{self, Write};
use std::fs;


pub fn display_once_from_file(file_path: &str) -> Result<(), ()> {
    crossterm::terminal::enable_raw_mode();
    print!("{}", crossterm::terminal::Clear(crossterm::terminal::ClearType::All));

    let result = match TaskList::load_tasks_from_csv(file_path) {
        Ok(task_list) => {
            print_task_list(&task_list);
            Ok(())
        },
        Err(e) => {
            println!("Failed to load tasks from file: {}", e);
            Err(())
        }
    };

    io::stdout().flush().unwrap();
    crossterm::terminal::disable_raw_mode();

    result
}

pub fn display_once(task_list: &TaskList) {
    crossterm::terminal::enable_raw_mode();
    print!("{}", crossterm::terminal::Clear(crossterm::terminal::ClearType::All));

    print_task_list(&task_list);

    io::stdout().flush().unwrap();
    crossterm::terminal::disable_raw_mode();
    println!("");
}


pub fn display_loop(file_path: &str) {
    println!("1");

    let initial_metadata = fs::metadata(&file_path)
        .expect("Failed to get file metadata");
    let mut last_modified: std::time::SystemTime = initial_metadata.modified()
        .expect("Failed to get modification time");

    match display_once_from_file(file_path) {
        Ok(_) => (),
        Err(_) => (
            return),
    }

    loop {
        if is_file_modified(&file_path, &mut last_modified) {
            let result = display_once_from_file(file_path);
            if result.is_err() {
                break;
            }
        }
        std::thread::sleep(
            std::time::Duration::from_millis(100)
        );
    }

    print!("");
}


fn print_task_list(task_list: &TaskList) {
    let max_name_width = task_list.max_name_width();
    let mut text: String;

    print_header(max_name_width + 8);
    print_line(max_name_width + 30);

    for i in 0..task_list.get_tasks().len() {
        print!("{}\r", crossterm::cursor::MoveTo(1, (i + 2) as u16));
        let task = &task_list.get_tasks()[i];

        text = {
            let mut text = format!(
                "{:<max_name_width$} | {:<10} | {:<10}",
                task.get_name(),
                task.get_priority(),
                task.get_deadline()
            );
            if *task.get_done() {
                text = green(strike(text));
            }
            text
        };

        println!("{}", text);
    }
}


fn print_header(head_size: usize) {
    print!("{}\r", crossterm::cursor::MoveTo(1, 0 as u16));
    let text = format!(
        "{:<head_size$} | {:<18} | {:<10}",
        bold("Task".to_string()),
        bold("Priority".to_string()),
        bold("Deadline".to_string())
    );
    println!("{}", text);
}


fn print_line(line_size: usize) {
    print!("{}\r", crossterm::cursor::MoveTo(1, 1 as u16));
    let repeated_string = "-".repeat(line_size);
    println!("{}", repeated_string);
}
