
use crate::utils::*;

pub struct Cli {
    tasks: TodoList,
}

impl Cli {
    pub fn new(date: String) -> Self {
        let file_path = format!("/history/{}.csv", date);
        Self {
            tasks: Self::load_tasks(&file_path),
        }
    }

    fn load_tasks(file_path: &str) -> TodoList {
        read(file_path).expect("Failed to read file")
    }

    pub fn add(&mut self, task_name: &str, priority_str: &str, due_date: &str) {

        let task = TodoTask::new(
            task_name.to_string(),
            Priority::new(priority_str),
            due_date.to_string()
        );
        self.tasks.add_task(task);
    }

    pub fn complete(&mut self, task_name: &str) {
        for task in self.tasks.get_tasks_mut() {
            if task.get_name() == task_name {
                task.set_done(true);
            }
        }
    }

    pub fn list(&self) {
        // Find the maximum width for each column
        let mut max_name_width = &self.tasks
            .get_tasks()
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(10);

        if max_name_width < &10 {
            max_name_width = &10;
        }

        let mut text: String;

        for task in self.tasks.get_tasks() {
            text = format!(
                "{:<max_name_width$} | {:<10} | {:<10}",
                task.get_name(),
                task.get_priority().to_string(),
                task.get_due_date(),
            );
            if *task.get_done() {
                text = green_strike(text.clone());
            }
            println!("{}", text);
        }
    }

    pub fn remove(&mut self, task_name: &str) {
        let mut index = 0;

        for task in self.tasks.get_tasks_mut() {
            if task.get_name() == task_name {
                self.tasks.get_tasks_mut().remove(index);
                return;
            }
            println!("{}, {}, {}", index, task.get_name(), task_name);
            index += 1;
        }

        println!("Task not found");
    }

    pub fn sort(&mut self) {
        self.tasks
            .get_tasks_mut()
            .sort_by(
                |a, b|b.get_priority().to_u8().cmp(&a.get_priority().to_u8())
            );
    }

}
