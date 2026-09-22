pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn area(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
    }
}
