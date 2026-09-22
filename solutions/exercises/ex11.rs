pub enum Wait {
    Seconds(u32),
    Minutes(u32),
}
pub fn seconds(wait: Wait) -> u64 {
    match wait {
        Wait::Seconds(n) => u64::from(n),
        Wait::Minutes(n) => u64::from(n) * 60,
    }
}
