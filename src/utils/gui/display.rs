
use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
};
use crate::{TaskList, strike, utils::*};
use std::io::{self, Write};


pub fn clear_all() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    stdout.flush()?;
    Ok(())
}


pub fn display_tasks_from_file(file_path: &str) -> Result<(), ()> {

    let task_list =  match TaskList::load_tasks_from_csv(file_path) {
        Ok(task_list) => task_list,
        Err(_) => return Err(())
    };

    display_tasks(&task_list)?;

    Ok(())
}


pub fn display_tasks(task_list: &TaskList) -> Result<(), ()>{
    let max_name_width = task_list.max_name_width();
    let mut text: String;

    print_header(max_name_width + 8);
    print_divider(max_name_width + 30);

    for i in 0..task_list.get_tasks().len() {
        let task = &task_list.get_tasks()[i];

        text = {
            let mut text = format!(
                " {:<2} | {:<max_name_width$} | {:<10} | {:<10}",
                i,
                task.get_name(),
                task.get_priority(),
                task.get_deadline()
            );
            if *task.get_done() {
                text = green(strike(text));
            }
            text
        };

        print_line(text, (i + 2) as u16);
    }
    println!("");

    Ok(())
}


pub fn print_header(head_size: usize) {
    print!("{}\r", cursor::MoveTo(1, 0 as u16));
    let text = format!(
        " {:<10} | {:<head_size$} | {:<18} | {:<10}",
        bold("ID".to_string()),
        bold("Task".to_string()),
        bold("Priority".to_string()),
        bold("Deadline".to_string())
    );
    println!("{}", text);
}


pub fn print_divider(line_size: usize) {
    print!("{}\r", cursor::MoveTo(1, 1 as u16));
    let repeated_string = "-".repeat(line_size);
    println!("{}", repeated_string);
}


pub fn print_line(msg: String, line: u16) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(
        stdout,
        "{}{}{}",
        cursor::MoveTo(0, line),
        Clear(ClearType::CurrentLine),
        msg
    )?;
    stdout.flush()?;
    Ok(())
}


pub fn update_view_screen(content: &str, file_path: &str) -> Result<(), io::Error> {
    update_screen(content, file_path)?;
    display_view_mode()?;
    Ok(())
}


pub fn update_command_screen(content: &str, input_buffer: &String, file_path: &str) -> Result<(), io::Error> {
    update_screen(content, file_path)?;
    display_command_mode(input_buffer)?;
    Ok(())
}


pub fn update_screen(content: &str, file_path: &str) -> Result<(), io::Error> {
    clear_all()?;
    match content {
        "help" => display_help()?,
        "task" => {
            // let file_path = "tasks.csv";
            let task_list = TaskList::load_tasks_from_csv(file_path)
                .expect("Failed to load tasks from file");
            display_tasks(&task_list);
        },
        _ => {}
    }
    Ok(())
}


pub fn display_help() -> Result<(), io::Error>{
    // TODO: Display help
    Ok(())
}


pub fn display_view_mode() -> Result<(), io::Error> {
    print_line(
        "VIEW MODE: Press 'i' to enter command mode.".to_string(),
        terminal::size()?.1 - 2
    )?;
    print_line(
        "Press Esc to quit the program.".to_string(),
        terminal::size()?.1 - 1
    )?;
    Ok(())
}


pub fn display_command_mode(input_buffer: &String) -> Result<(), io::Error> {
    print_line(
        "COMMAND MODE: Press 'Esc' to enter view mode.".to_string(),
        terminal::size()?.1 - 2
    )?;
    print_line(
        ":".to_string() + input_buffer,
        terminal::size()?.1 - 1
    )?;
    Ok(())
}
