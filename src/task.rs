use std::fmt::Display;
use std::process;
use serde::{Serialize, Deserialize};
use crate::file::save_json;
use crate::todo_list::TodoList;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
	pub name: String,
	pub description: String,
	pub date: String,
	pub completed: bool,
}

impl Display for Task {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let completed = if self.completed { "x" } else { " " };
		write!(f, "[{}] {} - {} - {}", completed, self.name, self.description, self.date)
	}
}

impl Task {
	pub fn new(name: String, description: String, date: String, completed: Option<bool>) -> Self {
		Self {
			name,
			description,
			date,
			completed: completed.unwrap_or(false),
		}
	}

	pub fn add(mut self, mut todo_list: TodoList) {
		let now = chrono::Local::now().format("%Y-%m-%d").to_string();
		if self.date.is_empty() { self.date = now; }

		let task = todo_list.find(&self.name);
		if task.is_some() {
			println!("Task \"{}\" already exists", task.unwrap().name);
			process::exit(1);
		}
		todo_list.tasks.push(self.clone());
		save_json(todo_list.tasks).unwrap();
		println!("Task \"{}\" added", self.name);
	}

	pub fn remove(self, mut todo_list: TodoList) {
		let list_len = todo_list.tasks.len();
		todo_list.retain(&self.name);
		let is_removed = list_len != todo_list.tasks.len();

		if is_removed {
			save_json(todo_list.tasks).unwrap();
			println!("Task \"{}\" removed", self.name);
		} else {
			println!("Task \"{}\" not found", self.name);
			process::exit(1);
		}
	}

	pub fn update(self, mut todo_list: TodoList, updated_task: Task) {
		todo_list.retain(&self.name);
		todo_list.tasks.push(updated_task);
		save_json(todo_list.tasks).unwrap();
		println!("Task \"{}\" updated", self.name);
	}

	pub fn complete(mut self, mut todo_list: TodoList) {
		self.completed = true;

		todo_list.retain(&self.name);
		todo_list.push(self.clone());
		save_json(todo_list.tasks).unwrap();
		println!("Task \"{}\" completed", self.name);
	}

	pub fn uncomplete(mut self, mut todo_list: TodoList) {
		self.completed = false;

		todo_list.retain(&self.name);
		todo_list.push(self.clone());
		save_json(todo_list.tasks).unwrap();
		println!("Task \"{}\" uncompleted", self.name);
	}

	pub fn status(self) {
		let status = if self.completed { "completed" } else { "uncompleted" };
		println!("Task \"{}\" is {}", self.name, status);
	}
}