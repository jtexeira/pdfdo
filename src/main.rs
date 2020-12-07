pub mod cli;
pub mod store;

use cli::{Categories, Commands};
use std::fs::File;
use std::io::BufReader;
use store::{cat::Category, task::Task, StoreMap};
use structopt::StructOpt;

fn main() {
    let args = Commands::from_args();

    let mut tasks_path = dirs::cache_dir().unwrap();
    tasks_path.push("pdfdo/tasks.json");
    let file = File::open(&tasks_path)
        .map(BufReader::new)
        .ok()
        .and_then(|x| serde_json::from_reader(x).ok());

    let mut cats_path = dirs::cache_dir().unwrap();
    cats_path.push("pdfdo/categories.json");
    let cat_reader = File::open(&cats_path)
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
            tasks.save(&tasks_path).expect("Can't save new task");
        }
        Commands::Get { id, url, pwd, file } => {
            if let Some(t) = tasks.get(&id) {
                if url && pwd && file || !url && !pwd && !file {
                    println!("{}", t.long_print());
                } else if pwd {
                    if let Some(p) = &t.get_work_dir() {
                        println!("{}", p.to_str().unwrap());
                    }
                } else if file {
                    if let Some(p) = &t.file {
                        println!("{}", p.to_str().unwrap());
                    }
                } else if url {
                    if let Some(p) = &t.url {
                        println!("{}", p.as_str());
                    }
                }
            } else {
                eprintln!("Inexistent task");
            }
        }
        Commands::Update {
            id,
            due_date,
            file,
            name,
            description,
            url,
            cat,
        } => {
            if let Some(t) = tasks.map.get_mut(&id) {
                t.update(due_date, file, name, description, url, cat);
                tasks.save(&tasks_path).expect("Can't save new task");
            }
        }
        Commands::Rm { id } => {
            tasks.rm(&id);
            tasks.save(&tasks_path).expect("Can't delete task");
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
                    cats.save(&cats_path).expect("Can't save new task");
                }
                Categories::List {} => print!("{}", cats),
                Categories::Rm { id } => {
                    cats.rm(&id);
                    cats.save(&cats_path).expect("Can't delete task");
                }
                Categories::Get { id, url, pwd } => {
                    if let Some(t) = cats.get(&id) {
                        if url && pwd || !url && !pwd {
                            println!("{}", t.long_print());
                        } else if pwd {
                            if let Some(p) = &t.work_dir {
                                println!("{}", p.to_str().unwrap());
                            }
                        } else if url {
                            if let Some(p) = &t.url {
                                println!("{}", p.as_str());
                            }
                        }
                    } else {
                        eprintln!("Inexistent task");
                    }
                }
            };
        }
    }
}
