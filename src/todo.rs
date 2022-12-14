use serde::{Serialize, Deserialize};
use crate::file::save_json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
	pub task: String,
	pub description: String,
	pub date: String,
	pub completed: bool,
}

impl Task {
	pub fn new(task: String, description: String, date: String) -> Self {
		Self {
			task,
			description,
			date,
			completed: false,
		}
	}

	pub fn add(self, mut todo_list: TodoList) {
		let task = todo_list.iter().find(|t| t.task == self.task);
		if task.is_some() {
			println!("Task \"{}\" already exists", task.unwrap().task);
			return;
		}
		todo_list.push(self.clone());
		save_json(todo_list).unwrap();
		println!("Task \"{}\" added", self.task);
	}

	pub fn remove(self, mut todo_list: TodoList) {
		let list_len = todo_list.len();
		todo_list.retain(|t| t.task != self.task);
		let is_removed = list_len != todo_list.len();

		if is_removed {
			save_json(todo_list).unwrap();
			println!("Task \"{}\" removed", self.task);
		} else {
			println!("Task \"{}\" not found", self.task);
		}
	}

	pub fn update(self, mut todo_list: TodoList) {
		let task = todo_list.iter_mut().find(|t| t.task == self.task);
		if task.is_some() {
			let task = task.unwrap();
			task.description = self.description;
			task.date = self.date;
			save_json(todo_list).unwrap();
			println!("Task \"{}\" updated", self.task);
		} else {
			println!("Task \"{}\" not found", self.task);
		}
	}

	pub fn complete(&mut self) {
		self.completed = true;
		println!("Task \"{}\" completed", self.task);
	}

	pub fn uncomplete(&mut self) {
		self.completed = false;
		println!("Task \"{}\" uncompleted", self.task);
	}
}

pub type TodoList = Vec<Task>;
