use std::error::Error;
use crate::utils::*;
use crate::utils::gui::*;


#[derive(Debug)]
pub struct TaskList {
    date: String,
    tasks: Vec<Task>,
}


impl TaskList {
    pub fn new(date: String, tasks: Vec<Task>) -> TaskList {
        TaskList {
            date,
            tasks,
        }
    }

    pub fn new_empty(date: String) -> TaskList {
        TaskList {
            date,
            tasks: Vec::new(),
        }
    }

    pub fn load_tasks_from_csv(file_path: &str) -> Result<TaskList, Box<dyn Error>> {
        match read(file_path) {
            Ok(task_list) => Ok(task_list),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn save_tasks_to_csv(&self, file_path: &str) {
        write(file_path, self).expect("Failed to write file");
    }

    pub fn add_task(&mut self, task_name: &str, priority: &str, deadline: &str) {

        let task = Task::new(
            task_name.to_string(),
            priority.to_string(),
            deadline.to_string()
        );

        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, task_name: &str) {
        let mut index = 0;

        for task in self.get_tasks_mut() {
            if task.get_name() == task_name {
                self.get_tasks_mut().remove(index);
                return;
            }
            index += 1;
        }

        println!("Task not found");
    }

    pub fn complete_task(&mut self, task_name: &str) {
        for task in self.get_tasks_mut() {
            if task.get_name() == task_name {
                task.set_done(true);
            }
        }
    }

    pub fn list_tasks(&self) {
        display_once(self);
    }

    pub fn sort_tasks(&mut self) {
        self.get_tasks_mut()
            .sort_by(
                |a, b|b.get_priority_u8().cmp(&a.get_priority_u8())
            );
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_tasks_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }

    pub fn max_name_width(&self) -> usize {
        let mut max_name_width = self.get_tasks()
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(10);

        if max_name_width < 10 {
            max_name_width = 10;
        }

        max_name_width
    }
}
