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

    pub fn add_task(&mut self, task: TodoTask) {
        self.tasks.push(task);
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_tasks(&self) -> &Vec<TodoTask> {
        &self.tasks
    }

    pub fn get_tasks_mut(&mut self) -> &mut Vec<TodoTask> {
        &mut self.tasks
    }
}
