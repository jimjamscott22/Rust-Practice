use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
static COUNTER: AtomicU64 = AtomicU64::new(0);
pub struct Sandbox(PathBuf);
impl Sandbox {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "rust-practice-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
    pub fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
