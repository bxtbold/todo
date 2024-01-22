// Module: io

use crate::task::{Task, DayTasks};


pub fn read(file_path: &str) -> Result<DayTasks, csv::Error> {

    let mut tasks: Vec<Task> = Vec::new();

    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        let task = Task::new(record);
        tasks.push(task);
    }

    Ok(DayTasks::new("20240121".to_string(), tasks))
}


pub fn write() {
    println!("Writing file...");
}
