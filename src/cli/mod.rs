mod cli;

pub use cli::*;


#[cfg(test)]
mod tests {

    use std::fs::{self, File};
    use tempfile::{tempdir, TempDir};
    use std::io::Write;
    use crate::*;

    fn create_tmp_file() -> (TempDir, String) {
        let tmp_dir = tempdir().expect("Failed to create temporary directory");
        let file_path_buf = tmp_dir.path().join("tasks.csv");
        let file_path = file_path_buf.to_str().unwrap();

        // write tasks to CSV file
        let mut file = std::fs::File::create(&file_path).expect("Failed to create file");
        write!(file, "is_completed,task_name,priority,deadline\n").unwrap();

        (tmp_dir, file_path.to_string())
    }

    fn create_cli(command: &str, task_name: &str, priority: &str, deadline: &str, id: usize) -> Cli {
        Cli {
            command: command.to_string(),
            task_name: task_name.to_string(),
            priority: priority.to_string(),
            deadline: deadline.to_string(),
            id: id,
        }
    }

    fn load_task_list_from_file(file_path: &str) -> TaskList {
        TaskList::load_tasks_from_csv(file_path).unwrap()
    }

    #[test]
    fn test_execute_add_command() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();

        let mut task_list = load_task_list_from_file(&tmp_file_path);

        // test command
        let cli = create_cli("add", "Buy groceries", "high", "2024-04-20", usize::MAX);
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        assert_eq!(task_list.get_tasks().len(), 1);
        assert_eq!(task_list.get_tasks()[0].get_name(), "Buy groceries");
    }

    #[test]
    fn test_execute_rm_command() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();

        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");


        // test command
        let cli = create_cli("rm", "Buy groceries", "", "", usize::MAX);
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        assert_eq!(task_list.get_tasks().len(), 2);
    }

    #[test]
    fn test_execute_rm_command_with_id() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();

        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");

        // test command
        // removing the task with id 1 which is Study in this test.
        let cli = create_cli("rm", "", "", "", 1);
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        for other_task in task_list.get_tasks() {
            assert_ne!(other_task.get_name(), &String::from("Study"));
        }
    }

    #[test]
    fn test_execute_done_command() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();

        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");

        // test command
        let cli = create_cli("done", "Buy groceries", "", "", usize::MAX);
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        for task in task_list.get_tasks() {
            if task.get_name() == "Buy groceries" {
                assert!(task.get_done());
            } else {
                assert!(!task.get_done());
            }
        }
    }

    #[test]
    fn test_execute_add_command_with_input_buffer() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();
        let mut task_list = load_task_list_from_file(&tmp_file_path);

        // test command
        let input_buffer = String::from("todo add Study");
        let cli = Cli::parse_input_buffer(&input_buffer).unwrap();
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        assert_eq!(task_list.get_tasks().len(), 1);
        assert_eq!(task_list.get_tasks().get(0).unwrap().get_name(), "Study");
    }


    #[test]
    fn test_execute_rm_command_with_input_buffer() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();
        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");

        // test command
        let input_buffer = String::from("todo rm Study");
        let cli = Cli::parse_input_buffer(&input_buffer).unwrap();
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        for task in task_list.get_tasks() {
            assert_ne!(task.get_name(), "Study");
        }
    }

    #[test]
    fn test_execute_rm_command_using_index_with_input_buffer() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();
        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");

        // test command
        let input_buffer = String::from("todo rm -i 1");
        let cli = Cli::parse_input_buffer(&input_buffer).unwrap();
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        for task in task_list.get_tasks() {
            assert_ne!(task.get_name(), "Study");
        }
    }

    #[test]
    fn test_execute_done_command_with_input_buffer() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();
        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");

        // test command
        let input_buffer = String::from("todo done Study");
        let cli = Cli::parse_input_buffer(&input_buffer).unwrap();
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        for task in task_list.get_tasks() {
            if task.get_name() == "Study" {
                assert!(task.get_done());
            } else {
                assert!(!task.get_done());
            }
        }
    }

    #[test]
    fn test_execute_done_command_using_index_with_input_buffer() {
        let (_tmp_dir, tmp_file_path) = create_tmp_file();
        let mut task_list = load_task_list_from_file(&tmp_file_path);
        task_list.add_task("Buy groceries", "mid", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-20");
        task_list.add_task("Do laundry", "low", "2024-04-20");

        // test command
        let input_buffer = String::from("todo done -i 1");
        let cli = Cli::parse_input_buffer(&input_buffer).unwrap();
        let result = cli.execute(&mut task_list);

        // test results
        assert!(result.is_ok());
        for task in task_list.get_tasks() {
            if task.get_name() == "Study" {
                assert!(task.get_done());
            } else {
                assert!(!task.get_done());
            }
        }
    }

}
