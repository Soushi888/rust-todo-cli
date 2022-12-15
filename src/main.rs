mod todo;
mod file;

use std::process;
use clap::Parser;
use crate::todo::*;
use crate::file::*;


#[derive(Debug, Parser)]
#[clap(author = "Soushi888", version)]
/// Simple todo cli app
struct Args {
	/// Action to do (add, remove, view, complete, uncomplete, status, clear)
	method: String,
	/// Task to do
	task: Option<String>,
	#[clap(short, long, default_value = "")]
	/// Description of the task
	description: String,
	/// Date of the task
	date: Option<String>,
}

fn main() {
	let args = Args::parse();

	let mut todo_list = if let Ok(todo_list) = load_json() {
		todo_list
	} else {
		process::exit(1);
	};

	match args.method.as_str() {
		"view" => {
			if todo_list.is_empty() {
				println!("No tasks");
				return;
			}

			for task in &todo_list {
				println!("{:#?}", task);
			}
		}
		"add" => {
			let task = Task::new(
				args.task.unwrap_or_else(|| {
					println!("Task is required");
					process::exit(1);
				}),
				args.description,
				args.date.unwrap_or_default());

			task.add(todo_list);
		}
		"remove" => {
			Task::new(
				args.task.unwrap_or("Undefined".to_string()),
				args.description,
				args.date.unwrap())
				.remove(todo_list);
		}
		"update" => {
			Task::new(
				args.task.unwrap_or_else(|| {
					println!("Task is required");
					process::exit(1);
				}),
				args.description,
				args.date.unwrap_or_default())
				.update(todo_list);
		}
		"complete" => {
			let task = args.task.clone().unwrap();
			let task = todo_list.iter_mut().find(|t| t.task == task);
			if task.is_some() {
				task.unwrap().complete();
				save_json(todo_list).unwrap();
			} else {
				println!("Task \"{}\" not found", args.task.unwrap());
			}
		}
		"uncomplete" => {
			let task = args.task.clone().unwrap();
			let task = todo_list.iter_mut().find(|t| t.task == task);
			if task.is_some() {
				task.unwrap().uncomplete();
				save_json(todo_list).unwrap();
			} else {
				println!("Task \"{}\" not found", args.task.unwrap());
			}
		}
		"status" => {
			let task = args.task.clone().unwrap();
			let task = todo_list.iter().find(|t| t.task == task);
			if task.is_some() {
				println!("Task \"{}\" is {}", task.unwrap().task,
								 if task.unwrap().completed { "completed" } else { "not completed" });
			} else {
				println!("Task \"{}\" not found", args.task.unwrap());
			}
		}
		"clear" => {
			save_json(Vec::new()).unwrap();
			println!("Todo list cleared");
		}
		_ => println!("Invalid command"),
	}
}