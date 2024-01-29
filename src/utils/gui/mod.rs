use crate::{TaskList, strike};
use std::io::{self, Write};


pub fn display_once(task_list: &TaskList) {
    crossterm::terminal::enable_raw_mode();
    print!("{}", crossterm::terminal::Clear(crossterm::terminal::ClearType::All));

    print_task_list(task_list);

    io::stdout().flush().unwrap();
    crossterm::terminal::disable_raw_mode();
    println!("");
}


pub fn display_loop(task_list: &TaskList) {
    crossterm::terminal::enable_raw_mode();
    print!("{}", crossterm::terminal::Clear(crossterm::terminal::ClearType::All));

    loop {
        display_once(task_list);
        std::thread::sleep(
            std::time::Duration::from_millis(100)
        );
    }

    io::stdout().flush().unwrap();
    crossterm::terminal::disable_raw_mode();
    print!("");
}


fn print_task_list(task_list: &TaskList) {
    let task_length = task_list.get_tasks().len();
    let max_name_width = task_list.max_name_width();

    for i in 0..task_length {
        print!("{}\r", crossterm::cursor::MoveTo(1, i as u16));
        let task = &task_list.get_tasks()[i];
        let mut text = format!(
            "{:<max_name_width$} | {:<10} | {:<10}",
            task.get_name(),
            task.get_priority().to_string(),
            task.get_deadline(),
        );
        if *task.get_done() {
            text = strike(text.clone());
        }
        println!("{}", text);
    }
}