pub mod cli;
pub mod store;

use cli::Commands;
use std::fs::File;
use std::io::BufReader;
use store::{task::Task, StoreMap};
use structopt::StructOpt;

fn main() {
    let args = Commands::from_args();
    let file = File::open("yeet.json")
        .map(BufReader::new)
        .ok()
        .and_then(|x| serde_json::from_reader(x).ok());

    let mut tasks: StoreMap<Task> = file.unwrap_or_default();
    match args {
        Commands::Add {
            due_date,
            file,
            name,
            description,
            url,
            cat,
        } => {
            tasks.add(Task::new(due_date, file, name, description, url, cat));
            tasks.save("yeet.json").expect("Can't save new task");
        }
        Commands::Get { id } => {
            if let Some(t) = tasks.get(id) {
                println!("{}: {}", id, t);
            } else {
                eprintln!("Inexistent task");
            }
        }
        Commands::Rm { id } => {
            tasks.rm(id);
            tasks.save("yeet.json").expect("Can't delete task");
        }
        Commands::List {} => println!("{}", tasks),
        _ => (),
    }
}
