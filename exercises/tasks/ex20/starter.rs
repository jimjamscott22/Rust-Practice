#![allow(unused_variables, unused_mut, dead_code)]
#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub name: String,
}
impl Record {
    pub fn parse(line: &str) -> Result<Self, String> {
        todo!("ex20: Records")
    }
    pub fn format(&self) -> String {
        todo!("ex20: format a record")
    }
}
