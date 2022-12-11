use std::fs;
use std::io::{Error};
use std::fs::{File};
use crate::todo::*;
use serde_json::{from_reader};

pub fn load_json() -> Result<TodoList, Error> {
    let file = File::open("todo.json").unwrap_or_else(|_| File::create("todo.json").unwrap());
    let todo_list: TodoList = from_reader(file).unwrap_or(vec![]);
    Ok(todo_list)
}

pub fn save_json(todo_list: TodoList) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(&todo_list).unwrap();
    fs::write("todo.json", json)?;

    Ok(())
}