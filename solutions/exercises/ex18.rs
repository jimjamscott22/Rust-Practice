pub trait Describe {
    fn describe(&self) -> String;
}
pub struct Book {
    pub title: String,
    pub author: String,
}
impl Describe for Book {
    fn describe(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
