use crate::model::{Task, TaskList};
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::Path;

pub fn decode(text: &str) -> Result<TaskList, String> {
    let mut tasks = Vec::new();
    for line in text.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err("expected ID, status, and title".into());
        }
        let id = fields[0].parse::<u32>().map_err(|_| "invalid ID")?;
        let done = match fields[1] {
            "0" => false,
            "1" => true,
            _ => return Err("invalid status".into()),
        };
        tasks.push(Task {
            id,
            done,
            title: fields[2].into(),
        });
    }
    TaskList::from_tasks(tasks)
}
pub fn encode(tasks: &TaskList) -> String {
    tasks
        .all()
        .iter()
        .map(|t| format!("{}\t{}\t{}\n", t.id, u8::from(t.done), t.title))
        .collect()
}
pub fn load(path: &Path) -> Result<TaskList, String> {
    match fs::read_to_string(path) {
        Ok(text) => decode(&text),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(TaskList::new()),
        Err(e) => Err(e.to_string()),
    }
}
pub fn save(path: &Path, tasks: &TaskList) -> Result<(), String> {
    let name = path.file_name().ok_or("file path has no filename")?;
    let mut temporary_name = name.to_os_string();
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    temporary_name.push(format!(".{}.{}.tmp", std::process::id(), nonce));
    let temporary = path.with_file_name(temporary_name);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| e.to_string())?;
    let result = (|| -> std::io::Result<()> {
        file.write_all(encode(tasks).as_bytes())?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temporary, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(|e| e.to_string())
}
