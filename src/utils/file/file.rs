use csv::WriterBuilder;
use std::error::Error;
use std::fs::{self, File};
use std::time::SystemTime;
use crate::utils::{Task, TaskList};


pub fn read(file_path: &str) -> Result<TaskList, csv::Error> {

    let mut tasks: Vec<Task> = Vec::new();

    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        let task = Task::new_from_csv(record);
        match task {
            Ok(task) => tasks.push(task),
            Err(e) => println!("{}", e),
        }
    }

    Ok(TaskList::new("20240121".to_string(), tasks))
}


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


pub fn check_today_file(file_path: &str) {
    if !file_exists(&file_path) {
        create_file(&file_path);
    }
}


pub fn get_today_date() -> String {
    let now = chrono::Local::now();
    now.format("%Y%m%d").to_string()
}


pub fn file_exists(file_path: &str) -> bool {
    std::path::Path::new(file_path).exists()
}


pub fn create_file(file_path: &str) {
    let file = std::fs::File::create(file_path)
        .expect("Failed to create file");
    write(file_path, &TaskList::new(get_today_date(), Vec::new()))
        .expect("Failed to write to file");
}
