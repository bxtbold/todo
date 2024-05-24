use clap::{Arg, ArgMatches, Command, Parser};
use crate::utils::*;


/// A struct representing the command line interface
#[derive(Parser, Debug)]
pub struct Cli {
    command: String,
    task_name: String,
    priority: String,
    deadline: String,
    id: usize
}


impl Cli {

    pub fn execute(&self, file_path: &str) -> Result<(), &str> {

        let mut task_list = match TaskList::load_tasks_from_csv(file_path) {
            Ok(task_list) => task_list,
            Err(_) => {
                return Err("Failed to load tasks from file");
            }
        };

        match self.command.as_str() {
            "add" => task_list.add_task(&self.task_name, &self.priority, &self.deadline),
            "rm" => {
                if (&self.task_name != "") {
                    task_list.remove_task(&self.task_name);
                }
                else if (self.id < usize::MAX) {
                    let task_name = task_list.get_task_name_with_id(&self.id);
                    task_list.remove_task(&task_name);
                }
            },
            "done" => {
                if (&self.task_name != "") {
                    task_list.complete_task(&self.task_name);
                }
                else if (self.id < usize::MAX) {
                    let task_name = task_list.get_task_name_with_id(&self.id);
                    task_list.complete_task(&task_name);
                }
            },
            "list" => task_list.list_tasks(),
            "sort" => task_list.sort_tasks(),
            "gui" => {
                let _ = display_gui(file_path);
                return Ok(());
            },
            _ => return Err("Invalid command"),
        }

        match task_list.save_tasks_to_csv(file_path) {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Failed to save tasks to the file!");
            }
        }
    }

    pub fn parse<'a>() -> Result<Cli, &'a str> {

        let matches = Cli::base_command()
            .subcommand(Cli::list_command())
            .subcommand(Cli::gui_command())
            .get_matches();

        return Cli::match_subcommands(matches);
    }

    pub fn parse_gui<'a>(match_vec: Vec<&str>) -> Result<Cli, &'a str> {

        let result_matches = Cli::base_command()
            .try_get_matches_from(match_vec);

        let matches = match result_matches {
            Ok(matches) => {
                return Cli::match_subcommands(matches);
            },
            Err(_) => {
                return Err("Invalid command");
            }
        };
    }


    pub fn base_command<'a>() -> Command {

        Command::new("todo")
            .about("Todo List Manager")
            .version("0.0.1")
            .author("Batbold N.")
            .override_usage("todo <COMMAND> [TASK_NAME] [OPTIONS]")
            .subcommand_required(true)
            .subcommand(Cli::add_command())
            .subcommand(Cli::rm_command())
            .subcommand(Cli::done_command())
            .subcommand(Cli::sort_command())
    }

    fn add_command() -> Command {
        Command::new("add")
            .about("Add a new task")
            .override_usage("todo add [TASK_NAME] [OPTIONS]")
            .arg(
                Arg::new("task_name")
                    .index(1)
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with("longer_name")
                    .required_unless_present("longer_name")
                    .help("The name of the task")
            )
            .arg(
                Arg::new("longer_name")
                    .short('n')
                    .long("name")
                    .value_parser(clap::value_parser!(String))
                    .num_args(1..=10)
                    .conflicts_with("task_name")
                    .help("The name of the task")
            )
            .arg(
                Arg::new("priority")
                    .short('p')
                    .long("priority")
                    .required(false)
                    .value_parser(clap::value_parser!(String))
                    .default_value("high")
                    .help("The priority of the task. Options: high, medium, low")
            )
            .arg(
                Arg::new("deadline")
                    .short('d')
                    .long("deadline")
                    .help("The deadline of the task. Format: YYYY-MM-DD")
                    .required(false)
                    .default_value("today")
                    .value_parser(clap::value_parser!(String))
            )
    }

    fn rm_command() -> Command {
        Command::new("rm")
            .about("Remove a task")
            .override_usage("todo rm [TASK_NAME]")
            .arg(
                Arg::new("task_name")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(["id", "longer_name"])
            )
            .arg(
                Arg::new("longer_name")
                    .short('n')
                    .long("name")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(["id"])
                    .help("The name of the task")
            )
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(["longer_name"])
                    .help("The ID of the task")
            )
    }

    fn done_command() -> Command {
        Command::new("done")
            .about("Complete a task")
            .override_usage("todo done [TASK_NAME]")
            .arg(
                Arg::new("task_name")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(["id", "longer_name"])
            )
            .arg(
                Arg::new("longer_name")
                    .short('n')
                    .long("name")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(["id"])
                    .help("The name of the task")
            )
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(["longer_name"])
                    .help("The ID of the task")
            )
    }

    fn list_command() -> Command {
        Command::new("list")
        .about("List all tasks")
        .override_usage("todo list")
    }

    fn sort_command() -> Command {
        Command::new("sort")
        .about("Sort tasks by priority")
        .override_usage("todo sort")
    }

    fn gui_command() -> Command {
        Command::new("gui")
        .about("A graphical user interface on the terminal")
        .override_usage("todo gui")
    }

    fn match_subcommands<'a>(matches: ArgMatches) -> Result<Cli, &'a str> {
        let mut command: String = String::from("");
        let mut task_name: String = String::from("");
        let mut priority: String = String::from("");
        let mut deadline: String = String::from("");
        let mut id: usize = std::usize::MAX;

        match matches.subcommand() {
            Some(("add", sub_matches))  => {
                command = "add".to_string();
                task_name = Cli::parse_subcommand_name(&sub_matches, "task_name");
                priority = Cli::parse_subcommand_name(&sub_matches, "priority");
                deadline = Cli::parse_subcommand_name(&sub_matches, "deadline");
            },
            Some(("rm", sub_matches))   => {
                command = "rm".to_string();
                match sub_matches.get_raw("task_name") {
                    Some(_task_name) => {
                        task_name = Cli::parse_subcommand_name(&sub_matches, "task_name");
                    },
                    _ => {}
                }
                match sub_matches.get_raw("id") {
                    Some(_) => {
                        let id_string = Cli::parse_subcommand_name(&sub_matches, "id");
                        match id_string.parse::<i32>() {
                            Ok(i) => { id = i as usize; },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            },
            Some(("done", sub_matches)) => {
                command = "done".to_string();
                match sub_matches.get_raw("task_name") {
                    Some(_task_name) => {
                        task_name = Cli::parse_subcommand_name(&sub_matches, "task_name");
                    },
                    _ => {}
                }
                match sub_matches.get_raw("id") {
                    Some(_) => {
                        let id_string = Cli::parse_subcommand_name(&sub_matches, "id");
                        match id_string.parse::<i32>() {
                            Ok(i) => { id = i as usize; },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            },
            Some(("list", sub_matches)) => {
                command = "list".to_string();
            },
            Some(("sort", sub_matches)) => {
                command = "sort".to_string();
            },
            Some(("gui", sub_matches)) => {
                command = "gui".to_string();
            },
            _ => {
                return Err("Invalid command");
            },
        }

        return Ok(
            Cli {
                command,
                task_name,
                priority,
                deadline,
                id,
            }
        );
    }

    fn parse_subcommand_name(matches: &ArgMatches, arg: &str) -> String {
        // longer name
        if arg == "task_name" {
            let longer_task_name = Cli::parse_longer_name(matches);
            if !longer_task_name.is_empty() {
                return longer_task_name;
            }
        }

        // shorter name
        matches
            .try_get_raw(arg)
            .expect("Could not read the value from the argument")
            .expect("Could not parse the value from the raw value")
            .into_iter()
            .next()
            .expect("Could not read the value to &OsStr")
            .to_str()
            .expect("Could not parse the value to &str")
            .to_string()
    }

    fn parse_longer_name(matches: &ArgMatches) -> String {
        matches
            .get_many::<String>("longer_name")
            .unwrap_or_default().map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}


#[cfg(test)]
mod tests {

    use std::fs::{self, File};
    use tempfile::tempdir;
    use std::io::Write;
    use super::*;

    fn create_tmp_file() -> String {
        let tmp_dir = tempdir().expect("Failed to create temporary directory");
        let file_path_buf = tmp_dir.path().join("tasks.csv");
        let file_path = file_path_buf.to_str().unwrap();

        // write tasks to CSV file
        let mut file = std::fs::File::create(&file_path).expect("Failed to create file");
        write!(file, "is_completed,task_name,priority,deadline\n").unwrap();

        file_path.to_string()
    }

    #[test]
    fn test_execute_add_command() {
        let tmp_file_path = create_tmp_file();

        let cli = Cli {
            command: "add".to_string(),
            task_name: "Buy groceries".to_string(),
            priority: "high".to_string(),
            deadline: "2024-04-20".to_string(),
            id: usize::MAX
        };

        let result = cli.execute(&tmp_file_path);
        // assert!(result.is_ok());

        let task_list = TaskList::load_tasks_from_csv(&tmp_file_path).expect("Failed to load tasks from file");
        assert_eq!(task_list.get_tasks().len(), 1);
        assert_eq!(task_list.get_tasks()[0].get_name(), "Buy groceries");
    }

    #[test]
    fn test_execute_rm_command() {
        let tmp_file_path = create_tmp_file();

        let mut task_list = TaskList::new_empty("2024-04-19".to_string());
        task_list.add_task("Buy groceries", "high", "2024-04-20");

        let cli = Cli {
            command: "rm".to_string(),
            task_name: "Buy groceries".to_string(),
            priority: "".to_string(),
            deadline: "".to_string(),
            id: usize::MAX
        };

        let result = cli.execute(&tmp_file_path);
        println!("rm result: {:?}", result);
        // assert!(result.is_ok());

        let task_list = TaskList::load_tasks_from_csv(&tmp_file_path).expect("Failed to load tasks from file");
        assert_eq!(task_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_execute_list_command() {
        let tmp_file_path = create_tmp_file();

        let mut task_list = TaskList::new_empty("2024-04-19".to_string());
        task_list.add_task("Buy groceries", "medium", "2024-04-20");
        task_list.add_task("Study", "high", "2024-04-25");

        let cli = Cli {
            command: "list".to_string(),
            task_name: "".to_string(),
            priority: "".to_string(),
            deadline: "".to_string(),
            id: usize::MAX
        };

        let result = cli.execute(&tmp_file_path);
        // assert!(result.is_ok());

    }

    #[test]
    fn test_execute_rm_with_index_command() {
        let tmp_file_path = create_tmp_file();

        let mut task_list = TaskList::new_empty("2024-05-09".to_string());
        task_list.add_task("Study", "high", "2024-05-09");
        task_list.add_task("Buy groceries", "mid", "2024-05-09");

        let cli = Cli {
            command: "rm".to_string(),
            task_name: "".to_string(),
            priority: "".to_string(),
            deadline: "".to_string(),
            id: 0 as usize
        };

        println!("cli: {:?}", cli);
        println!("tmp_file_path: {:?}", tmp_file_path);
        let result = cli.execute(&tmp_file_path);
        // assert!(result.is_ok());

        let task_list = TaskList::load_tasks_from_csv(&tmp_file_path).expect("Failed to load tasks from file");
        assert_eq!(task_list.get_tasks().len(), 0);

        let remaning_task = task_list.get_tasks().get(0).unwrap().get_name();
        assert_eq!(remaning_task, "Buy groceries");
    }

}