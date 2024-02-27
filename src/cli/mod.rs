use clap::{Arg, ArgMatches, Command, Parser};
use crate::utils::*;


/// A struct representing the command line interface
#[derive(Parser, Debug)]
pub struct Cli {
    command: String,
    task_name: String,
    priority: String,
    deadline: String,
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
            "rm" => task_list.remove_task(&self.task_name),
            "done" => task_list.complete_task(&self.task_name),
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
            .required(true)
            .value_parser(clap::value_parser!(String))
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
            .required(true)
            .value_parser(clap::value_parser!(String))
            .help("The name of the task")
        )
    }

    fn done_command() -> Command {
        Command::new("done")
        .about("Complete a task")
        .override_usage("todo done [TASK_NAME]")
        .arg(
            Arg::new("task_name")
            .required(true)
            .value_parser(clap::value_parser!(String))
            .help("The name of the task")
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

        match matches.subcommand() {
            Some(("add", sub_matches))  => {
                command = "add".to_string();
                task_name = Cli::parse_subcommand_name(&sub_matches, "task_name");
                priority = Cli::parse_subcommand_name(&sub_matches, "priority");
                deadline = Cli::parse_subcommand_name(&sub_matches, "deadline");
            },
            Some(("rm", sub_matches))   => {
                command = "rm".to_string();
                task_name = Cli::parse_subcommand_name(&sub_matches, "task_name");
            },
            Some(("done", sub_matches)) => {
                command = "done".to_string();
                task_name = Cli::parse_subcommand_name(&sub_matches, "task_name");
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
            }
        );
    }

    fn parse_subcommand_name(matches: &ArgMatches, arg: &str) -> String {
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
}
