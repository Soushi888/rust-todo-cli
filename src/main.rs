mod todo;
mod file;

use clap::Parser;
use chrono;
use crate::todo::*;
use crate::file::*;


#[derive(Debug, Parser)]
#[clap(author = "Soushi888", version)]
/// Simple todo cli app
struct Args {
	/// Action to do (add, remove, view, complete, uncomplete, status)
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
	let mut args = Args::parse();
	let now = chrono::Local::now().format("%Y-%m-%d").to_string();
	if args.date.is_none() { args.date = Some(now); }

	let mut todo_list: TodoList = load_json().unwrap();
	match args.method.as_str() {
		"view" => {
			for task in &todo_list {
				println!("{:#?}", task);
			}
		}
		"add" => {
			let task = Task::new(
				args.task.unwrap_or("Undefined".to_string()),
				args.description,
				args.date.unwrap());

			task.add(todo_list);
		}
		"remove" => {
			let task = Task::new(
				args.task.unwrap_or("Undefined".to_string()),
				args.description,
				args.date.unwrap());

			task.remove(todo_list);
		}
		"update" => {
			let task = Task::new(
				args.task.unwrap_or("Undefined".to_string()),
				(args.description),
				args.date.unwrap());

			task.update(todo_list);
		}
		"complete" => {
			let task = args.task.unwrap();
			let task = todo_list.iter_mut().find(|t| t.task == task).unwrap();
			task.complete();
			save_json(todo_list).unwrap();
		}
		"uncomplete" => {
			let task = args.task.unwrap();
			let task = todo_list.iter_mut().find(|t| t.task == task).unwrap();
			task.uncomplete();
			save_json(todo_list).unwrap();
		}
		"status" => {
			let task = args.task.unwrap();
			let task = todo_list.iter().find(|t| t.task == task).unwrap();
			println!("Task \"{}\" is {}", task.task, if task.is_completed() { "completed" } else { "not completed" });
		}
		_ => println!("Invalid command"),
	}
}