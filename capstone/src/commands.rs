#![allow(unused_variables)]
use std::path::PathBuf;
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add(String),
    List,
    Done(u32),
}
#[derive(Debug, PartialEq, Eq)]
pub struct Options {
    pub file: PathBuf,
    pub command: Command,
}
pub fn parse(args: &[String]) -> Result<Options, String> {
    todo!("milestone 3: parse arguments")
}
