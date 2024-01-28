use clap::{builder::TypedValueParser, Arg, ArgMatches, Command, Parser};
use crate::utils::*;


/// A struct representing the command line interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "todo")]
pub struct Cli {
    command: String,
    task_name: String,
    priority: String,
    deadline: String,
}


impl Cli {

    pub fn execute(&self, file_path: &str) -> Result<TaskList, &str> {
        let mut task_list = TaskList::load_tasks_from_csv(file_path);

        match self.command.as_str() {
            "add" => task_list.add_task(&self.task_name, &self.priority, &self.deadline),
            "rm" => task_list.remove_task(&self.task_name),
            "done" => task_list.complete_task(&self.task_name),
            "list" => task_list.list_tasks(),
            "sort" => task_list.sort_tasks(),
            _ => return Err("Invalid command"),
        }

        task_list.save_tasks_to_csv(file_path);

        Ok(task_list)
    }

    pub fn parse<'a>() -> Result<Cli, &'a str> {

        let matches = Command::new("todo")
            .about("Todo List Manager")
            .version("0.0.1")
            .author("Batbold N.")
            .override_usage("todo <COMMAND> [TASK_NAME] [OPTIONS]")
            .subcommand_required(true)
            .subcommand(Cli::add_command())
            .subcommand(Cli::rm_command())
            .subcommand(Cli::done_command())
            .subcommand(Cli::list_command())
            .subcommand(Cli::sort_command())
            .get_matches();

        Cli::match_subcommands(matches)
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
