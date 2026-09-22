use capstone::{
    commands::{self, Command},
    storage,
};
fn run() -> Result<(), String> {
    let options = commands::parse(&std::env::args().skip(1).collect::<Vec<_>>())?;
    let mut tasks = storage::load(&options.file)?;
    match options.command {
        Command::Add(title) => {
            let id = tasks.add(title)?;
            storage::save(&options.file, &tasks)?;
            println!("Added task {id}.");
        }
        Command::List => {
            if tasks.all().is_empty() {
                println!("No tasks.");
            }
            for task in tasks.all() {
                println!(
                    "{} [{}] {}",
                    task.id,
                    if task.done { "x" } else { " " },
                    task.title
                );
            }
        }
        Command::Done(id) => {
            tasks.complete(id)?;
            storage::save(&options.file, &tasks)?;
            println!("Completed task {id}.");
        }
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
