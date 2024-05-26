use std::error::Error;
use crate::utils::*;
use crate::utils::gui::*;


#[derive(Debug)]
pub struct TaskList {
    date: String,
    tasks: Vec<Task>,
    env: Environment
}


impl TaskList {
    pub fn new(date: String, tasks: Vec<Task>, file_path: String) -> TaskList {
        let env = Environment::new(file_path);
        TaskList {
            date,
            tasks,
            env,
        }
    }

    pub fn new_empty(date: String) -> TaskList {
        TaskList {
            date,
            tasks: Vec::new(),
            env: Environment::new(String::new()),
        }
    }

    pub fn update_list_from_csv(&mut self) {
        let task_list = TaskList::load_tasks_from_csv(self.get_env().get_file_path()).unwrap();
        self.tasks = task_list.tasks;
    }

    pub fn load_tasks_from_csv(file_path: &str) -> Result<TaskList, Box<dyn Error>> {
        match read(file_path) {
            Ok(task_list) => Ok(task_list),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn save_tasks_to_csv(&self) -> Result<(), Box<dyn Error>> {
        let file_path = self.get_env().get_file_path();
        write(file_path, self)?;
        Ok(())
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
        let mut index: usize = 0;

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
        display_tasks(self);
    }

    pub fn sort_tasks(&mut self) {
        self.get_tasks_mut()
            .sort_by(
                |a, b|b.get_priority_u8().cmp(&a.get_priority_u8())
            );
    }

    pub fn get_task_name_with_id(&self, id: &usize) -> String {
        let task_list = self.get_tasks();
        match task_list.get(*id) {
            Some(task) => {
                return task.get_name().to_owned()
            },
            _ => return String::from("")
        }
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

    pub fn get_env(&self) -> &Environment {
        &self.env
    }

    pub fn get_env_mut(&mut self) -> &mut Environment {
        &mut self.env
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
