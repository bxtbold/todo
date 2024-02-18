use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, Clear, ClearType},
};
use std::{fs, io::{self, Write}, time::Duration};
use crate::{is_file_modified, utils::gui::display::*};


pub fn handle_event(prev_size: &mut (u16, u16), event: Event, file_path: &str) -> Result<(), io::Error> {
    match event {
        Event::Key(KeyEvent { code, .. }) => {
            match code {
                KeyCode::Char('I') | KeyCode::Char('i') => handle_command_mode(prev_size, file_path)?,
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


pub fn handle_command_mode(prev_size: &mut (u16, u16), file_path: &str) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    let mut input_buffer = String::new();

    update_command_screen("task", &input_buffer, file_path)?;
    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                if let Event::Key(KeyEvent { code, .. }) = event {
                    match code {
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Enter => {
                            input_buffer.clear();
                            write!(stdout, "{}", Clear(ClearType::UntilNewLine))?;
                        }
                        KeyCode::Char(c) => {
                            input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            input_buffer.pop();
                        }
                        _ => {}
                    }

                    write!(
                        stdout,
                        "{}{}:{}",
                        cursor::MoveTo(0, terminal::size()?.1 - 1),
                        Clear(ClearType::CurrentLine),
                        input_buffer
                    )?;
                    stdout.flush()?;
                }
            }
        }


        let initial_metadata = fs::metadata(&file_path)
        .expect("Failed to get file metadata");
        let mut last_modified: std::time::SystemTime = initial_metadata.modified()
            .expect("Failed to get modification time");

        let current_size = terminal::size()?;
        if current_size != *prev_size || is_file_modified(file_path, &mut last_modified) {
            update_command_screen("task", &input_buffer, file_path)?;
            *prev_size = current_size;
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
