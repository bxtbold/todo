mod task;
mod task_list;

pub use task::Task;
pub use task_list::TaskList;

// unittests for Task
#[test]
fn test_task_creation() {
    let task = Task::new("Buy groceries".to_string(), "high".to_string(), "2024-04-30".to_string());
    assert_eq!(task.get_name(), "Buy groceries");
    assert_eq!(task.get_priority(), "high");
    assert_eq!(task.get_deadline(), "2024-04-30");
    assert_eq!(*task.get_done(), false);
}

#[test]
fn test_task_csv_format() {
    let task = Task::new("Study for exam".to_string(), "medium".to_string(), "2024-05-15".to_string());
    assert_eq!(task.csv_format(), "false,Study for exam,medium,2024-05-15");
}

#[test]
fn test_task_set_done() {
    let mut task = Task::new("Clean room".to_string(), "low".to_string(), "2024-04-25".to_string());
    task.set_done(true);
    assert_eq!(*task.get_done(), true);
}

#[test]
fn test_task_set_priority() {
    let mut task = Task::new("Write report".to_string(), "medium".to_string(), "2024-04-28".to_string());
    task.set_priority("high");
    assert_eq!(task.get_priority(), "high");
}

#[test]
fn test_task_get_priority_u8() {
    let task_high = Task::new("Task".to_string(), "high".to_string(), "2024-04-20".to_string());
    let task_medium = Task::new("Task".to_string(), "medium".to_string(), "2024-04-20".to_string());
    let task_low = Task::new("Task".to_string(), "low".to_string(), "2024-04-20".to_string());
    let task_unknown = Task::new("Task".to_string(), "unknown".to_string(), "2024-04-20".to_string());

    assert_eq!(task_high.get_priority_u8(), 3);
    assert_eq!(task_medium.get_priority_u8(), 2);
    assert_eq!(task_low.get_priority_u8(), 1);
    assert_eq!(task_unknown.get_priority_u8(), 0);
}

#[test]
fn test_task_len() {
    let task = Task::new("Finish project".to_string(), "high".to_string(), "2024-05-10".to_string());
    assert_eq!(task.len(), 14);
}

// unittests for TaskList
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_task_list_creation() {
        let task_list = TaskList::new("2024-04-19".to_string(), vec![], String::new());
        assert_eq!(task_list.get_date(), "2024-04-19");
        assert_eq!(task_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_task_list_new_empty() {
        let task_list = TaskList::new_empty("2024-04-19".to_string());
        assert_eq!(task_list.get_date(), "2024-04-19");
        assert_eq!(task_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut task_list = TaskList::new_empty("2024-04-19".to_string());
        task_list.add_task("Buy groceries", "high", "2024-04-20");
        assert_eq!(task_list.get_tasks().len(), 1);
        assert_eq!(task_list.get_tasks()[0].get_name(), "Buy groceries");
    }

    #[test]
    fn test_remove_task() {
        let mut task_list = TaskList::new_empty("2024-04-19".to_string());
        task_list.add_task("Buy groceries", "high", "2024-04-20");
        task_list.remove_task("Buy groceries");
        assert_eq!(task_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_complete_task() {
        let mut task_list = TaskList::new_empty("2024-04-19".to_string());
        task_list.add_task("Buy groceries", "high", "2024-04-20");
        task_list.complete_task("Buy groceries");
        assert_eq!(*task_list.get_tasks()[0].get_done(), true);
    }

    #[test]
    fn test_sort_tasks() {
        let mut task_list = TaskList::new_empty("2024-04-19".to_string());
        task_list.add_task("Task 1", "high", "2024-04-20");
        task_list.add_task("Task 2", "low", "2024-04-20");
        task_list.sort_tasks();
        assert_eq!(task_list.get_tasks()[0].get_priority(), "high");
        assert_eq!(task_list.get_tasks()[1].get_priority(), "low");
    }
}
