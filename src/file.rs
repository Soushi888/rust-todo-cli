use std::io::Error;
use std::fs::{File};
use crate::todo::*;
use serde_json::{from_reader};

pub fn load_json() -> Result<TodoList, Error> {
    let file = File::open("todo.json").unwrap_or_else(|_| File::create("todo.json").unwrap());
    let todo_list: TodoList = from_reader(file).unwrap_or(vec![]);
    Ok(todo_list)
}

pub fn save_json(todo_list: TodoList) -> Result<(), Error> {
    // let file = File::create("./todo2.json").unwrap_or_else(|err| {
    //     panic!("Error creating file: {}", err);
    // });
    // serde_json::to_writer_pretty(file, &todo_list).unwrap_or_else(|err| {
    //     panic!("Error writing file: {}", err);
    // });

    Ok(())
}