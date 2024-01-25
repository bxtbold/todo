// This module contains the module to read.

use crate::utils::{TodoTask, TodoList};


pub fn read(file_path: &str) -> Result<TodoList, csv::Error> {

    let mut tasks: Vec<TodoTask> = Vec::new();

    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        let task = TodoTask::new_from_csv(record);
        tasks.push(task);
    }

    Ok(TodoList::new("20240121".to_string(), tasks))
}
