pub struct TodoTask {
    done: bool,
    name: String,
    priority: String,
    due_date: String,
}


impl TodoTask {
    pub fn new(record: csv::StringRecord) -> TodoTask {
        let done = record.get(0).unwrap().parse::<bool>().unwrap();
        let name = record.get(1).unwrap().to_string();
        let priority = record.get(2).unwrap().to_string();
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

    pub fn get_priority(&self) -> &String {
        &self.priority
    }

    pub fn get_due_date(&self) -> &String {
        &self.due_date
    }
}
