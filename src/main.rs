pub mod cli;
pub mod store;

use cli::{Categories, Commands};
use std::fs::File;
use std::io::BufReader;
use store::{cat::Category, task::Task, StoreMap};
use structopt::StructOpt;

fn main() {
    let args = Commands::from_args();
    let file = File::open("yeet.json")
        .map(BufReader::new)
        .ok()
        .and_then(|x| serde_json::from_reader(x).ok());

    let cat_reader = File::open("categories.json")
        .map(BufReader::new)
        .ok()
        .and_then(|x| serde_json::from_reader(x).ok());
    let mut tasks: StoreMap<usize, Task> = file.unwrap_or_default();
    let mut cats: StoreMap<String, Category> = cat_reader.unwrap_or_default();
    match args {
        Commands::Add {
            due_date,
            file,
            name,
            description,
            url,
            cat,
        } => {
            tasks.add_us(Task::new(due_date, file, name, description, url, cat));
            tasks.save("yeet.json").expect("Can't save new task");
        }
        Commands::Get { id } => {
            if let Some(t) = tasks.get(&id) {
                println!("{}: {}", id, t.long_print());
            } else {
                eprintln!("Inexistent task");
            }
        }
        Commands::Rm { id } => {
            tasks.rm(&id);
            tasks.save("yeet.json").expect("Can't delete task");
        }
        Commands::List {} => print!("{}", tasks),
        Commands::Cat { cat } => {
            match cat {
                Categories::Add {
                    name,
                    description,
                    url,
                    work_dir,
                } => {
                    cats.add(
                        name.clone(),
                        Category::new(&name, description, url, work_dir),
                    );
                    cats.save("categories.json").expect("Can't save new task");
                }
                Categories::List {} => print!("{}", cats),
                Categories::Rm { id } => {
                    cats.rm(&id);
                    cats.save("yeet.json").expect("Can't delete task");
                }
                Categories::Get { id } => {
                    if let Some(t) = cats.get(&id) {
                        println!("{}: {}", &id, t.long_print());
                    } else {
                        eprintln!("Inexistent task");
                    }
                }
                _ => (),
            };
        }
        _ => (),
    }
}
