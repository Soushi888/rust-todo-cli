mod todo;
mod file;

use std::process;
use clap::Parser;
use chrono;
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
	let mut args = Args::parse();
	let now = chrono::Local::now().format("%Y-%m-%d").to_string();
	if args.date.is_none() { args.date = Some(now); }

	let mut todo_list = match load_json() {
		Ok(todo_list) => todo_list,
		Err(e) => {
			println!("Error: {}", e);
			process::exit(1);
		}
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
				args.task.unwrap_or("Undefined".to_string()),
				args.description,
				args.date.unwrap());

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
				args.task.unwrap_or("Undefined".to_string()),
				args.description,
				args.date.unwrap())
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