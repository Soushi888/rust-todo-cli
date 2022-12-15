# rust-todo-cli

A simple todo cli app written in Rust.

## Usage

To use the app, run the `rust-todo-cli` command followed by one of the available subcommands, as shown below:

`rust-todo-cli <COMMAND>`

## Available Commands

- `view`: View all tasks.
    - Examples:
        - `rust-todo-cli view`
- `add`: Add a new task.
    - Arguments:
        - `<TASK_NAME>`: The name of the task to add.
    - Options:
        - `[DESCRIPTION]`: The description of the task to add.
        - `[DATE]`: The due date of the task to add.
    - Examples:
        - `rust-todo-cli add "Buy milk"`
        - `rust-todo-cli add "Buy milk" "Pick up milk from the store" "2022-12-15"`
- `remove`: Remove a task.
    - Arguments:
        - `<TASK_NAME>`: The name of the task to remove.
    - Examples:
        - `rust-todo-cli remove "Buy milk"`
- `update`: Update a task.
    - Arguments:
        - `<TASK_NAME>`: The name of the task to update.
    - Options:
        - `--name <NEW_NAME>`: The new name for the task.
        - `--description <DESCRIPTION>`: The new description for the task.
        - `--date <DATE>`: The new due date for the task.
        - `--completed <COMPLETED>`: Whether the task is completed or not (possible values: `true`, `false`).
    - Examples:
        - `rust-todo-cli update "Buy milk" --name "Get groceries" --description "Pick up milk and bread" --date "2022-12-15"`
- `complete`: Complete a task.
    - Arguments:
        - `<TASK_NAME>`: The name of the task to complete.
    - Examples:
        - `rust-todo-cli complete "Buy milk"`
- `uncomplete`: Uncomplete a task.
    - Arguments:
        - `<TASK_NAME>`: The name of the task to uncomplete.
    - Examples:
        - `rust-todo-cli uncomplete "Buy milk"`
- `status`: Show the status of the task.
    - Arguments:
        - `<TASK_NAME>`: The name of the task to show the status of.
    - Examples:
        - `rust-todo-cli status "Buy milk"`
- `clear-completed`: Clear completed tasks.
    - Examples:
        - `rust-todo-cli clear-completed`
- `clear-all`: Clear all tasks.
    - Examples:
        - `rust-todo-cli clear-all`

## Options

- `-h, --help`: Print help information
- `-V, --version`: Print version information

