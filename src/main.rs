//! # rust-todo-cli
//!
//! A simple todo cli app written in Rust.
//!
//! ## Usage
//!
//! To use the app, run the `rust-todo-cli` command followed by one of the available subcommands, as shown below:
//!
//! `rust-todo-cli <COMMAND>`
//!
//! ## Available Commands
//!
//! - `view`: View all tasks.
//!     - Examples:
//!         - `rust-todo-cli view`
//! - `add`: Add a new task.
//!     - Arguments:
//!         - `<TASK_NAME>`: The name of the task to add.
//!     - Options:
//!         - `[DESCRIPTION]`: The description of the task to add.
//!         - `[DATE]`: The due date of the task to add.
//!     - Examples:
//!         - `rust-todo-cli add "Buy milk"`
//!         - `rust-todo-cli add "Buy milk" "Pick up milk from the store" "2022-12-15"`
//! - `remove`: Remove a task.
//!     - Arguments:
//!         - `<TASK_NAME>`: The name of the task to remove.
//!     - Examples:
//!         - `rust-todo-cli remove "Buy milk"`
//! - `update`: Update a task.
//!     - Arguments:
//!         - `<TASK_NAME>`: The name of the task to update.
//!     - Options:
//!         - `--name <NEW_NAME>`: The new name for the task.
//!         - `--description <DESCRIPTION>`: The new description for the task.
//!         - `--date <DATE>`: The new due date for the task.
//!         - `--completed <COMPLETED>`: Whether the task is completed or not (possible values: `true`, `false`).
//!     - Examples:
//!         - `rust-todo-cli update "Buy milk" --name "Get groceries" --description "Pick up milk and bread" --date "2022-12-15"`
//! - `complete`: Complete a task.
//!     - Arguments:
//!         - `<TASK_NAME>`: The name of the task to complete.
//!     - Examples:
//!         - `rust-todo-cli complete "Buy milk"`
//! - `uncomplete`: Uncomplete a task.
//!     - Arguments:
//!         - `<TASK_NAME>`: The name of the task to uncomplete.
//!     - Examples:
//!         - `rust-todo-cli uncomplete "Buy milk"`
//! - `status`: Show the status of the task.
//!     - Arguments:
//!         - `<TASK_NAME>`: The name of the task to show the status of.
//!     - Examples:
//!         - `rust-todo-cli status "Buy milk"`
//! - `clear-completed`: Clear completed tasks.
//!     - Examples:
//!         - `rust-todo-cli clear-completed`
//! - `clear-all`: Clear all tasks.
//!     - Examples:
//!         - `rust-todo-cli clear-all`
//!
//! ## Options
//!
//! - `-h, --help`: Print help information
//! - `-V, --version`: Print version information

mod todo_list;
mod file;
mod task;

use std::process;
use clap::{Args, Parser, Subcommand};
use crate::todo_list::*;
use crate::task::*;
use crate::file::*;


#[derive(Debug, Parser)]
#[clap(author = "Soushi888", version)]
/// Simple todo cli app
struct Cli {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
	/// View all tasks
	View,
	/// Add a new task
	Add(AddTaskArgs),
	/// Remove a task
	Remove(TaskNameArg),
	/// Update a task
	Update(UpdateTaskArgs),
	/// Complete a task
	Complete(TaskNameArg),
	/// Uncomplete a task
	Uncomplete(TaskNameArg),
	/// Show the status of the task
	Status(TaskNameArg),
	/// Clear completed tasks
	ClearCompleted,
	/// Clear all tasks
	ClearAll,
}

#[derive(Args, Debug, Clone)]
struct AddTaskArgs {
	/// Task name
	name: String,
	/// Task description
	description: Option<String>,
	/// Task date
	date: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct TaskNameArg {
	/// Task name
	name: String,
}

#[derive(Args, Debug, Clone)]
struct UpdateTaskArgs {
	name: String,
	#[clap(long, short)]
	new_name: Option<String>,
	#[clap(long, short)]
	description: Option<String>,
	#[clap(long)]
	date: Option<String>,
	#[clap(long, short)]
	completed: Option<bool>,
}

use Commands::*;

fn main() {
	let cli = Cli::parse();

	let mut todo_list = if let Ok(todo_list) = load_json() {
		todo_list
	} else {
		process::exit(1);
	};

	match cli.command {
		View => { todo_list.list(); }
		Add(args) => {
			let task = Task::new(
				args.name,
				args.description.unwrap_or_default(),
				args.date.unwrap_or_default(),
				None,
			);
			task.add(todo_list);
		}
		Remove(args) => {
			let task = Task::new(
				args.name,
				String::new(),
				String::new(),
				None,
			);
			task.remove(todo_list);
		}
		Update(args) => {
			if args.new_name.is_none() && args.description.is_none()
				&& args.date.is_none() && args.completed.is_none() {
				println!("No value to update");
				process::exit(1);
			}

			let task = get_task(&todo_list, &args.name);
			let new_task = Task::new(
				args.new_name.unwrap_or(args.name),
				args.description.unwrap_or(task.description.clone()),
				args.date.unwrap_or(task.date.clone()),
				Some(args.completed.unwrap_or(task.completed)),
			);
			task.update(todo_list, new_task);
		}
		Complete(args) => {
			let task = get_task(&todo_list, &args.name);
			task.complete(todo_list);
		}
		Uncomplete(args) => {
			let task = get_task(&todo_list, &args.name);
			task.uncomplete(todo_list);
		}
		Status(args) => {
			let task = get_task(&todo_list, &args.name);
			task.status();
		}
		ClearCompleted => {
			todo_list.clear_completed();
		}
		ClearAll => {
			todo_list.clear_all();
			save_json(todo_list.tasks).unwrap();
		}
	}
}

fn get_task(todo_list: &TodoList, task_name: &String) -> Task {
	if let Some(task) = todo_list.find(task_name) {
		task.clone()
	} else {
		println!("Task \"{}\" not found", task_name);
		process::exit(1);
	}
}