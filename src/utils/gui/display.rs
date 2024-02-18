
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


pub fn display_once_from_file(file_path: &str) -> Result<(), ()> {
    terminal::enable_raw_mode();
    print!("{}", terminal::Clear(terminal::ClearType::All));

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
    terminal::disable_raw_mode();

    result
}


pub fn display_once(task_list: &TaskList) {
    terminal::enable_raw_mode();
    print!("{}", terminal::Clear(terminal::ClearType::All));

    print_task_list(&task_list);

    io::stdout().flush().unwrap();
    terminal::disable_raw_mode();
    println!("");
}


pub fn print_task_list(task_list: &TaskList) {
    let max_name_width = task_list.max_name_width();
    let mut text: String;

    print_header(max_name_width + 8);
    print_line(max_name_width + 30);

    for i in 0..task_list.get_tasks().len() {
        print!("{}\r", cursor::MoveTo(1, (i + 2) as u16));
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


pub fn print_header(head_size: usize) {
    print!("{}\r", cursor::MoveTo(1, 0 as u16));
    let text = format!(
        "{:<head_size$} | {:<18} | {:<10}",
        bold("Task".to_string()),
        bold("Priority".to_string()),
        bold("Deadline".to_string())
    );
    println!("{}", text);
}


pub fn print_line(line_size: usize) {
    print!("{}\r", cursor::MoveTo(1, 1 as u16));
    let repeated_string = "-".repeat(line_size);
    println!("{}", repeated_string);
}


pub fn update_screen(content: &str, file_path: &str) -> Result<(), io::Error> {
    clear_all()?;
    match content {
        "help" => display_help()?,
        "task" => {
            // let file_path = "tasks.csv";
            let task_list = TaskList::load_tasks_from_csv(file_path)
                .expect("Failed to load tasks from file");
            display_tasks(&task_list)?;
        },
        _ => {}
    }
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


pub fn display_tasks(task_list: &TaskList) -> Result<(), io::Error> {
    let terminal_size = terminal::size()?.1 as usize;
    let max_tasks_to_display = terminal_size - 2;
    let num_tasks = task_list.get_tasks().len();
    let num_tasks_to_display = if num_tasks > max_tasks_to_display {
        max_tasks_to_display
    } else {
        num_tasks
    };

    let max_name_width = task_list.max_name_width();
    print_header(max_name_width + 8);
    print_line(max_name_width + 30);

    let mut stdout = io::stdout();
    let mut text: String;

    for i in 0..num_tasks_to_display {
        let task = &task_list.get_tasks()[i];
        let j = i + 2;
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
        write!(
            stdout,
            "{}{}{}",
            cursor::MoveTo(0, j as u16),
            Clear(ClearType::CurrentLine),
            text
        )?;
        stdout.flush()?;
    }
    Ok(())
}


pub fn display_help() -> Result<(), io::Error>{
    // TODO: Display help
    Ok(())
}


pub fn display_view_mode() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(
        stdout,
        "{}{}VIEW MODE: Press 'i' to enter command mode.{}{}Press Esc to quit the program.",
        cursor::MoveTo(0, terminal::size()?.1 - 2),
        Clear(ClearType::CurrentLine),
        cursor::MoveTo(0, terminal::size()?.1 - 1),
        Clear(ClearType::CurrentLine)
    )?;
    stdout.flush()?;
    Ok(())
}


pub fn display_command_mode(input_buffer: &String) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(
        stdout,
        "{}{}COMMAND MODE: Press 'Esc' to enter view mode.{}:{}",
        cursor::MoveTo(0, terminal::size()?.1 - 2),
        Clear(ClearType::CurrentLine),
        cursor::MoveTo(0, terminal::size()?.1 - 1),
        input_buffer
    )?;
    stdout.flush()?;
    Ok(())
}
