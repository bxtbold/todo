
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Priority {
    pub fn new(priority: &str) -> Priority {
        match priority {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => Priority::Low,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self.to_string() {
            "low" => 1,
            "medium" => 2,
            "high" => 3,
            _ => 1,
        }
    }
}