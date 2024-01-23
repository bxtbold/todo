use crate::utils::TodoTask;

pub struct TodoList {
    date: String,
    tasks: Vec<TodoTask>,
}


impl TodoList {
    pub fn new(date: String, tasks: Vec<TodoTask>) -> TodoList {
        TodoList {
            date,
            tasks,
        }
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_tasks(&self) -> &Vec<TodoTask> {
        &self.tasks
    }
}
