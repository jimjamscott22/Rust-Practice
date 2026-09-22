#![allow(unused_variables)]
use crate::model::TaskList;
use std::path::Path;
pub fn decode(text: &str) -> Result<TaskList, String> {
    todo!("milestone 4: parse storage")
}
pub fn encode(tasks: &TaskList) -> String {
    todo!("milestone 4: format storage")
}
pub fn load(path: &Path) -> Result<TaskList, String> {
    todo!("milestone 4: load storage")
}
pub fn save(path: &Path, tasks: &TaskList) -> Result<(), String> {
    todo!("milestone 4: save storage")
}
