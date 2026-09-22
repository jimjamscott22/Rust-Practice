use crate::model::validate_title;
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
    let (file, rest) = if args.first().is_some_and(|s| s == "--file") {
        let path = args
            .get(1)
            .filter(|s| !s.is_empty())
            .ok_or("missing file path")?;
        (PathBuf::from(path), &args[2..])
    } else {
        (PathBuf::from("tasks.tsv"), args)
    };
    let command = match rest {
        [verb] if verb == "list" => Command::List,
        [verb, title] if verb == "add" => {
            validate_title(title)?;
            Command::Add(title.clone())
        }
        [verb, id] if verb == "done" => {
            let id: u32 = id.parse().map_err(|_| "ID must be a positive u32")?;
            if id == 0 {
                return Err("ID must be positive".into());
            }
            Command::Done(id)
        }
        _ => return Err("usage: task-lab [--file PATH] add \"TITLE\" | list | done ID".into()),
    };
    Ok(Options { file, command })
}
