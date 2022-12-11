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
    task: String,
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

    let mut todo_list: TodoList = vec![];

    if args.method == "add" {
        let task = Task::new(args.task, args.description, args.date.unwrap());
        todo_list.push(task);
    }

    let json = load_json().unwrap();
    dbg!(&json);
}