use std::fs;
use std::io::{Error};
use std::fs::{OpenOptions};
use crate::todo::*;
use serde_json::{from_reader, to_writer};

pub fn load_json() -> Result<TodoList, Error> {
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open("todo.json")?;

	let todo_list = if file.metadata()?.len() == 0 {
		let todo_list = Vec::new();
		to_writer(file, &todo_list)?;
		todo_list
	} else {
		from_reader(file)?
	};

	Ok(todo_list)
}

pub fn save_json(mut todo_list: TodoList) -> Result<(), Error> {
	todo_list.sort_by_key(|t| t.date.clone());
	let json = serde_json::to_string_pretty(&todo_list).unwrap();
	fs::write("todo.json", json)?;

	Ok(())
}