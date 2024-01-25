use crate::Priority;

pub struct TodoTask {
    done: bool,
    name: String,
    priority: Priority,
    due_date: String,
}


impl TodoTask {

    pub fn new(name: String, priority: Priority, due_date: String) -> TodoTask {
        TodoTask {
            done: false,
            name,
            priority,
            due_date,
        }
    }

    pub fn new_from_csv(record: csv::StringRecord) -> TodoTask {
        let done = record.get(0).unwrap().parse::<bool>().unwrap();
        let name = record.get(1).unwrap().to_string();
        let priority = Priority::new(&record.get(2).unwrap().to_string());
        let due_date = record.get(3).unwrap().to_string();
        TodoTask {
            done,
            name,
            priority,
            due_date,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.name.len()
    }

    pub fn get_done(&self) -> &bool {
        &self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn get_priority(&self) -> Priority {
        self.priority
    }

    pub fn get_priority_u8(&self) -> u8 {
        self.priority.to_u8()
    }

    pub fn set_priority(&mut self, priority_str: &str) {
        self.priority = Priority::new(priority_str);
    }

    pub fn get_due_date(&self) -> &String {
        &self.due_date
    }

    pub fn set_due_date(&mut self, due_date: String) {
        self.due_date = due_date;
    }
}
