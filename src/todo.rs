use serde::{Serialize, Deserialize};

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
}

pub type TodoList = Vec<Task>;
