// This module contains the module to read.

use crate::utils::{Task, TaskList};


pub fn read(file_path: &str) -> Result<TaskList, csv::Error> {

    let mut tasks: Vec<Task> = Vec::new();

    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        let task = Task::new_from_csv(record);
        tasks.push(task);
    }

    Ok(TaskList::new("20240121".to_string(), tasks))
}
