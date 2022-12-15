use serde::{Serialize, Deserialize};
use crate::file::save_json;
use crate::task::Task;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoList {
	pub tasks: Vec<Task>,
}

impl TodoList {
	pub fn new() -> Self {
		Self {
			tasks: Vec::new(),
		}
	}

	pub fn list(&self) {
		if self.tasks.is_empty() {
			println!("No tasks found");
			return;
		}
		for task in &self.tasks {
			println!("{}", task);
		}
	}

	pub fn find(&self, name: &String) -> Option<&Task> {
		self.tasks.iter().find(|t| t.name == *name)
	}

	pub fn retain(&mut self, name: &String) {
		self.tasks.retain(|t| t.name != *name);
	}

	pub fn push(&mut self, task: Task) {
		self.tasks.push(task);
	}

	pub fn clear_completed(&mut self) {
		self.tasks.retain(|t| !t.completed);
		save_json(self.tasks.clone()).unwrap();
		println!("Completed tasks cleared");
	}

	pub fn clear_all(&mut self) {
		self.tasks.clear();
		println!("Todo list cleared");
	}
}