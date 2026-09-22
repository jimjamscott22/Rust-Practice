#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub active: bool,
}
pub fn new_user(name: String) -> User {
    User { name, active: true }
}
