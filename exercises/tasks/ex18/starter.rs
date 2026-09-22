#![allow(unused_variables, unused_mut, dead_code)]
pub trait Describe {
    fn describe(&self) -> String;
}
pub struct Book {
    pub title: String,
    pub author: String,
}
impl Describe for Book {
    fn describe(&self) -> String {
        todo!("ex18: Traits")
    }
}
