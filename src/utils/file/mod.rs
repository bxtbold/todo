use std::fs::{self, File};

mod file;
pub use file::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_and_write_tasks() {
        // create temporary directory
        let tmp_dir = tempdir().expect("Failed to create temporary directory");

        // create temporary CSV file path
        let file_path_buf = tmp_dir.path().join("tasks.csv");
        let file_path = file_path_buf.to_str().unwrap();

        // write tasks to CSV file
        let mut file: File = std::fs::File::create(&file_path).expect("Failed to create file");
        write!(file, "is_completed,task_name,priority,deadline\ntrue,Buy groceries,high,2024-04-20\nfalse,Study,medium,2024-04-25").unwrap();

        // read tasks from CSV file
        let task_list = read(&file_path).expect("Failed to read tasks from file");

        // verify tasks read correctly
        assert_eq!(task_list.get_tasks().len(), 2);
        assert_eq!(task_list.get_tasks()[0].get_name(), "Buy groceries");
        assert_eq!(task_list.get_tasks()[1].get_name(), "Study");

        // write tasks back to CSV file
        write(&file_path, &task_list).expect("Failed to write tasks to file");

        // read tasks again to verify write operation
        let updated_task_list = read(&file_path).expect("Failed to read tasks from file");
        assert_eq!(updated_task_list.get_tasks().len(), 2);
        assert_eq!(updated_task_list.get_tasks()[0].get_name(), "Buy groceries");
        assert_eq!(updated_task_list.get_tasks()[1].get_name(), "Study");
    }

    #[test]
    fn test_check_today_file() {
        let tmp_dir = tempdir().expect("Failed to create temporary directory");
        let file_path_buf = tmp_dir.path().join("tasks.csv");
        let file_path = file_path_buf.to_str().unwrap();

        // call check_today_file, which should create the file
        check_today_file(&file_path);

        // check that the file exists after calling check_today_file
        assert!(file_exists(&file_path));
    }

    #[test]
    fn test_get_today_date() {
        let today_date = get_today_date();
        // verify that the date format is correct (YYYYMMDD)
        assert_eq!(today_date.len(), 8);
    }

    #[test]
    fn test_file_exists() {
        // create temporary directory
        let tmp_dir = tempdir().expect("Failed to create temporary directory");
        let file_path = tmp_dir.path().join("test.txt");

        // Ensure that file doesn't exist initially
        let file_path_buf = tmp_dir.path().join("tasks.csv");
        let file_path = file_path_buf.to_str().unwrap();

        // create file
        let _file = fs::File::create(&file_path).expect("Failed to create file");

        // check that the file exists after creation
        assert!(file_exists(&file_path));
    }
}
