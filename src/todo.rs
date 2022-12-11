use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    task: String,
    description: String,
    date: String,
    completed: bool,
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
}

pub type TodoList = Vec<Task>;
