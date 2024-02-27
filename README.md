# Todo

The Vim-inspired todo tool is a command-line interface (CLI) application designed to provide likable interactions for managing daily tasks. With its intuitive commands and minimalist design, todo empowers users to effortlessly prioritize and accomplish their tasks.

Whether you're a Vim user seeking a familiar environment or someone in search of a fast and efficient tool for task management, todo offers a simple yet powerful solution.

## Prerequisites

- [Rust Installation](https://www.rust-lang.org/tools/install)

## Build from Source

To build todo from the source, follow these steps:

```bash
rustup update
git clone https://github.com/bxtbold/todo.git
cd todo
cargo build --release
echo "source $PWD/scripts/todo.sh" >> ~/.bashrc   # replace .bashrc with .zshrc if your machine uses zshell
```

This will update rust, clone the repository, build the todo tool, and enable the tab-completion.

## Usage

```
Usage: todo <COMMAND> [TASK_NAME] [OPTIONS]

Commands:
  add   Add a new task
  rm    Remove a task
  done  Complete a task
  sort  Sort tasks by priority
  list  List all tasks
  gui   A graphical user interface on the terminal
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

todo is open-source software licensed under the MIT License.
Contributions are very much welcome, so feel free to fork the repository and submit pull requests.

