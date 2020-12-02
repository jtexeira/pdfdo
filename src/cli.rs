use chrono::naive::NaiveDate;
use std::path::PathBuf;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
pub enum Commands {
    List {},
    Get {
        id: usize,
        #[structopt(long)]
        url: bool,
        #[structopt(long)]
        pwd: bool,
        #[structopt(long)]
        file: bool,
    },
    Add {
        name: String,
        #[structopt(long)]
        cat: Option<String>,
        #[structopt(long)]
        due_date: Option<NaiveDate>,
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
    Get {
        id: String,
        #[structopt(long)]
        url: bool,
        #[structopt(long)]
        pwd: bool,
    },
    List {},
    Rm {
        id: String,
    },
}
