// This module contains the module to write files.
use crate::utils::TaskList;

use std::error::Error;
use std::fs::File;
use csv::WriterBuilder;


pub fn write(file_path: &str, task_list: &TaskList) -> Result<(), Box<dyn Error>> {

    let file = File::create(file_path)?;
    let mut csv_writer = WriterBuilder::new().from_writer(file);
    csv_writer.write_record(&["is_completed", "task_name", "priority", "deadline"])?;
    for task in task_list.get_tasks() {
        // Write each field separately, not as a single string
        csv_writer.write_record(&[
            &task.get_done().to_string(),
            &task.get_name(),
            &task.get_priority(),
            &task.get_deadline(),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
