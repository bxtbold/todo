use chrono::prelude::*;

struct Date {
    year: u32,
    month: u32,
    day: u32,
}


impl Date {

    pub fn new() -> Self {
        let local: DateTime<Local> = Local::now();
        let year = local.year() as u32;
        let month = local.month();
        let day = local.day();
        Self {
            year,
            month,
            day,
        }
    }

    pub fn get_year(&self) -> u32 {
        self.year
    }

    pub fn get_month(&self) -> u32 {
        self.month
    }

    pub fn get_day(&self) -> u32 {
        self.day
    }

    pub fn get_date(&self) -> String {
        format!("{}_{}_{}", self.year, self.month, self.day)
    }
}
