
use crate::utils::*;

pub struct Cli {
}

impl Cli {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_today(&self, file_path: &str) {
        let day_tasks = read(file_path)
            .expect("Failed to read file");

        // // Find the maximum width for each column
        let mut task_width = day_tasks.get_tasks().iter().map(|s| s.len()).max().unwrap_or(10);
        if task_width < 10 {
            task_width = 10;
        }

        let mut text: String;

        for task in day_tasks.get_tasks() {
            text = format!(
                "{:<task_width$} | {:<10} | {:<10}",
                task.get_name(),
                task.get_priority(),
                task.get_due_date(),
            );
            if *task.get_done() {
                text = green_strike(text.clone());
            }
            println!("{}", text);
        }
    }
}
