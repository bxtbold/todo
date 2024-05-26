use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, Clear, ClearType},
};
use std::{
    fs,
    io::{self, Write},
    time::Duration,
};
use crate::{
    cli::Cli, utils::gui::{display::*, environment::*}, TaskList
};


pub fn handle_event(task_list: &mut TaskList, event: Event) -> Result<(), io::Error> {
    match event {
        Event::Key(KeyEvent { code, .. }) => {
            match code {
                KeyCode::Char('I') | KeyCode::Char('i') => handle_command_mode(task_list)?,
                KeyCode::Esc => {
                    return Err(io::Error::new(io::ErrorKind::Other, "Esc pressed"));
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}


pub fn handle_command_mode(task_list: &mut TaskList) -> Result<(), io::Error> {
    let mut input_buffer = String::new();

    update_command_screen("task", &input_buffer, task_list)?; // immutable borrow ends here

    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => handle_enter_key(&mut input_buffer, task_list)?,
                            KeyCode::Char(c) => input_buffer.push(c),
                            KeyCode::Backspace => { input_buffer.pop(); }
                            _ => {}
                        }

                        print_line(format!(":{}", input_buffer), terminal::size()?.1 - 1)?;                    }
                    _ => {}
                }
            }
        }

        if task_list.get_env_mut().is_file_modified() {
            task_list.update_list_from_csv();
            update_command_screen("task", &input_buffer, task_list)?;
        }
        if task_list.get_env_mut().is_terminal_resized() {
            update_command_screen("task", &input_buffer, task_list)?;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}


fn update_and_handle_input(task_list: &mut TaskList, input_buffer: &mut String) -> Result<(), io::Error> {

    update_command_screen("task", input_buffer, task_list)?; // immutable borrow ends here

    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => handle_enter_key(input_buffer, task_list)?,
                            KeyCode::Char(c) => input_buffer.push(c),
                            KeyCode::Backspace => { input_buffer.pop(); }
                            _ => {}
                        }

                        print_line(format!(":{}", input_buffer), terminal::size()?.1 - 1)?;                    }
                    _ => {}
                }
            }
        }

        if task_list.get_env_mut().is_file_modified() {
            task_list.update_list_from_csv();
            update_command_screen("task", &input_buffer, task_list)?;
        }
        if task_list.get_env_mut().is_terminal_resized() {
            update_command_screen("task", &input_buffer, task_list)?;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}


fn handle_enter_key(input_buffer: &mut String, task_list: &mut TaskList) -> Result<(), io::Error> {
    let cli = Cli::parse_input_buffer(&input_buffer).unwrap();
    if cli.execute(task_list).is_ok() {
        task_list.save_tasks_to_csv();
    }
    input_buffer.clear();
    clear_line()?;
    Ok(())
}
