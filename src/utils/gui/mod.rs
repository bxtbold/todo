mod display;
mod environment;
mod handle;

use crate::{Task, TaskList};
use std::{fs, io};
use crossterm::{
    event, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;

pub use display::*;
pub use environment::*;
pub use handle::*;


pub fn display_gui(task_list: &mut TaskList) -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    gui_loop(task_list)?;

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}


fn gui_loop(task_list: &mut TaskList)-> Result<(), io::Error> {
    loop {

        update_view_screen("task", task_list)?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                if let Err(_) = handle_event(task_list, event) {
                    break;
                }
            }
        }

        if task_list.get_env_mut().is_file_modified() {
            task_list.update_list_from_csv();
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
