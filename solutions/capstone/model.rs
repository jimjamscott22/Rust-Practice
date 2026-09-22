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
    if title.trim().is_empty() || title.contains(['\t', '\n', '\r']) {
        Err("title must be nonblank and contain no tabs or line breaks".into())
    } else {
        Ok(())
    }
}

impl TaskList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    pub fn all(&self) -> &[Task] {
        &self.tasks
    }
    pub fn from_tasks(tasks: Vec<Task>) -> Result<Self, String> {
        let mut ids = std::collections::HashSet::new();
        for task in &tasks {
            validate_title(&task.title)?;
            if task.id == 0 || !ids.insert(task.id) {
                return Err("IDs must be positive and unique".into());
            }
        }
        Ok(Self { tasks })
    }
    pub fn add(&mut self, title: String) -> Result<u32, String> {
        validate_title(&title)?;
        let id = self
            .tasks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or("no IDs remaining")?;
        self.tasks.push(Task {
            id,
            title,
            done: false,
        });
        Ok(id)
    }
    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or("unknown task ID")?;
        task.done = true;
        Ok(())
    }
}
impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}
