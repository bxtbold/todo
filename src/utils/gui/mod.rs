mod handle;
mod display;

use std::io;
use crossterm::{
    event, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;

pub use handle::*;
pub use display::*;


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
        let mut prev_size = terminal::size()?;
        update_view_screen("task", file_path)?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                if let Err(_) = handle_event(&mut prev_size, event, file_path) {
                    break;
                }
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
