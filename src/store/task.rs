use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    #[serde(with = "ts_seconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub file: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cat: Option<String>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cat.as_ref().map(|x| write!(f, "[{}] ", x));
        writeln!(f, "{}", self.name);
        Ok(())
    }
}

impl Task {
    pub fn new(
        due_date: Option<DateTime<Utc>>,
        file: Option<PathBuf>,
        name: String,
        description: Option<String>,
        url: Option<Url>,
        cat: Option<String>,
    ) -> Self {
        Task {
            due_date,
            file: file.map(|x| x.canonicalize().unwrap()),
            name,
            description,
            url,
            cat,
        }
    }
    pub fn short_print(&self) -> String {
        let mut string = String::new();
        self.cat
            .as_ref()
            .map(|x| string.push_str(format!("[{}] ", x).as_str()));
        string.push_str(format!("{}: ", self.name).as_str());
        string
    }
    pub fn long_print(&self) -> String {
        let mut string = format!("{}\n", self.name);
        self.cat
            .as_ref()
            .map(|x| string.push_str(format!("Category: {}", x).as_str()));
        self.due_date
            .map(|x| string.push_str(format!("Due To: {}", x).as_str()));
        self.description
            .as_ref()
            .map(|x| string.push_str(format!("Description: {}", x).as_str()));
        self.file
            .as_ref()
            .map(|x| string.push_str(format!("File: {}", x.to_str().unwrap()).as_str()));
        self.url
            .as_ref()
            .map(|x| string.push_str(format!("Url: {}", x).as_str()));
        string
    }
}
