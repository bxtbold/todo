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
    cli::Cli, utils::gui::{display::*, environment::*}
};


pub fn handle_event(env: &mut Environment, event: Event) -> Result<(), io::Error> {
    match event {
        Event::Key(KeyEvent { code, .. }) => {
            match code {
                KeyCode::Char('I') | KeyCode::Char('i') => handle_command_mode(env)?,
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


pub fn handle_command_mode(env: &mut Environment) -> Result<(), io::Error> {
    let mut input_buffer = String::new();

    let file_path = env.get_file_path().to_owned(); // immutable borrow here
    update_command_screen("task", &input_buffer, &file_path)?; // immutable borrow ends here

    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => handle_enter_key(&mut input_buffer, &file_path)?,
                            KeyCode::Char(c) => input_buffer.push(c),
                            KeyCode::Backspace => { input_buffer.pop(); }
                            _ => {}
                        }

                        print_line(format!(":{}", input_buffer), terminal::size()?.1 - 1)?;                    }
                    _ => {}
                }
            }
        }

        if env.should_update_view() {
            let file_path = env.get_file_path();
            update_command_screen("task", &input_buffer, file_path)?;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}


fn update_and_handle_input(env: &mut Environment, input_buffer: &mut String) -> Result<(), io::Error> {

    let file_path = env.get_file_path().to_owned(); // immutable borrow here
    update_command_screen("task", input_buffer, &file_path)?; // immutable borrow ends here

    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(KeyEvent { code, .. }) => {
                        match code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => handle_enter_key(input_buffer, &file_path)?,
                            KeyCode::Char(c) => input_buffer.push(c),
                            KeyCode::Backspace => { input_buffer.pop(); }
                            _ => {}
                        }

                        print_line(format!(":{}", input_buffer), terminal::size()?.1 - 1)?;                    }
                    _ => {}
                }
            }
        }

        if env.should_update_view() {
            let file_path = env.get_file_path();
            update_command_screen("task", input_buffer, file_path)?;
            env.update_modified_time();
            env.update_terminal_size();
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}


fn handle_enter_key(input_buffer: &mut String, file_path: &str) -> Result<(), io::Error> {
    let matches: Vec<&str> = input_buffer.split_whitespace().collect();
    let cli = Cli::parse_gui(matches.clone());
    match cli {
        Ok(cli) => {cli.execute(file_path);}
        Err(e) => {}
    }
    input_buffer.clear();
    clear_line()?;
    Ok(())
}
