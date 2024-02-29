mod display;
mod environment;
mod handle;

use std::{fs, io};
use crossterm::{
    event, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;

pub use display::*;
pub use environment::*;
pub use handle::*;


pub fn display_gui(file_path: &str) -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    gui_loop(file_path)?;

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}


fn gui_loop(file_path: &str)-> Result<(), io::Error> {
    loop {
        let mut env = Environment::new(file_path);

        update_view_screen("task", file_path)?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                if let Err(_) = handle_event(&mut env, event) {
                    break;
                }
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
