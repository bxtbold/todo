#[derive(Debug)]
pub struct Task {
    done: bool,
    name: String,
    priority: String,
    deadline: String,
}


impl Task {

    pub fn new(name: String, priority: String, deadline: String) -> Task {
        Task {
            done: false,
            name,
            priority,
            deadline,
        }
    }

    pub fn new_from_csv(record: csv::StringRecord) -> Task {
        let done = record.get(0).unwrap().parse::<bool>().unwrap();
        let name = record.get(1).unwrap().to_string();
        let priority = record.get(2).unwrap().to_string();
        let deadline = record.get(3).unwrap().to_string();
        Task {
            done,
            name,
            priority,
            deadline,
        }
    }

    pub fn csv_format(&self) -> String {
        format!("{},{},{},{}", self.done, self.name, self.priority, self.deadline)
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

    pub fn get_priority(&self) -> &String {
        &self.priority
    }

    pub fn get_priority_u8(&self) -> u8 {
        match self.priority.as_str() {
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        }
    }

    pub fn set_priority(&mut self, priority: &str) {
        self.priority = priority.to_string();
    }

    pub fn get_deadline(&self) -> &String {
        &self.deadline
    }

    pub fn set_deadline(&mut self, deadline: String) {
        self.deadline = deadline;
    }
}
