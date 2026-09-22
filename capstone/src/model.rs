#![allow(unused_variables, dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub struct TaskList {
    tasks: Vec<Task>,
}
pub fn validate_title(title: &str) -> Result<(), String> {
    todo!("milestone 2: validate title")
}
impl TaskList {
    pub fn new() -> Self {
        todo!("milestone 1: empty list")
    }
    pub fn all(&self) -> &[Task] {
        todo!("milestone 1: borrow tasks")
    }
    pub fn from_tasks(tasks: Vec<Task>) -> Result<Self, String> {
        todo!("milestone 4: validate loaded tasks")
    }
    pub fn add(&mut self, title: String) -> Result<u32, String> {
        todo!("milestone 2: add task")
    }
    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        todo!("milestone 2: complete task")
    }
}
impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}
