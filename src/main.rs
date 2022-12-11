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
    /// Action to do (add, remove, view, complete)
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

            todo_list.push(task.clone());
            save_json(todo_list).unwrap();
            println!("Task {} added", task.task);
        }
        _ => println!("Invalid command"),
    }
}