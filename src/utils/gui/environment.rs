use std::{
    fs,
    io,
    time::SystemTime,
};
use crossterm::terminal;

#[derive(Debug)]
pub struct Environment {
    file_path: String,
    prev_terminal_size: (u16, u16),
    prev_modified_time: SystemTime,
}

impl Environment {
    pub fn new(file_path: String) -> Environment {
        let mut prev_terminal_size = (0, 0);
        let mut prev_modified_time = SystemTime::now();
        Environment {
            file_path,
            prev_terminal_size,
            prev_modified_time
        }
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn get_prev_terminal_size(&self) -> &(u16, u16) {
        &self.prev_terminal_size
    }

    pub fn get_prev_modified_time(&self) -> &SystemTime {
        &self.prev_modified_time
    }

    pub fn update_modified_time(&mut self) {
        let initial_metadata = fs::metadata(&self.file_path)
            .expect("Failed to retrieve file metadata!");
        self.prev_modified_time = initial_metadata.modified()
            .expect("Failed to read the current modified time!");
    }

    pub fn update_terminal_size(&mut self) {
        self.prev_terminal_size = terminal::size().unwrap();
    }

    pub fn is_file_modified(&self) -> bool {
        let initial_metadata = fs::metadata(&self.file_path)
            .expect("Failed to retrieve file metadata!");
        let last_modified = initial_metadata.modified()
            .expect("Failed to read the current modified time!");

        self.prev_modified_time != last_modified
    }

    pub fn is_terminal_resized(&self) -> bool {
        let current_size = terminal::size().unwrap();

        self.prev_terminal_size != current_size
    }

    pub fn should_update_view(&mut self) -> bool {
        let result = self.is_file_modified() || self.is_terminal_resized();

        if result {
            self.update_modified_time();
            self.update_terminal_size();
        }

        result
    }
}
