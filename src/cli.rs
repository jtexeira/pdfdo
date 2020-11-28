use chrono::{DateTime, Utc};
use std::path::PathBuf;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
pub enum Commands {
    List {},
    Get {
        id: usize,
    },
    File {
        id: usize,
    },
    Url {
        id: usize,
    },
    Add {
        name: String,
        #[structopt(long)]
        cat: Option<String>,
        #[structopt(long)]
        due_date: Option<DateTime<Utc>>,
        #[structopt(long)]
        file: Option<PathBuf>,
        #[structopt(long)]
        description: Option<String>,
        #[structopt(long)]
        url: Option<Url>,
    },
    Rm {
        id: usize,
    },
    Cat {
        #[structopt(subcommand)]
        cat: Categories,
    },
}

#[derive(StructOpt)]
pub enum Categories {
    Add {
        name: String,
        #[structopt(long)]
        description: Option<String>,
        #[structopt(long)]
        url: Option<Url>,
        #[structopt(long)]
        work_dir: Option<PathBuf>,
    },
    List {},
    Rm {
        id: usize,
    },
    Url {
        id: usize,
    },
    Dir {
        id: usize,
    },
}