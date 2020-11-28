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
        writeln!(f, "{}", self.name)?;
        self.cat.as_ref().map(|x| writeln!(f, "Category: {}", x));
        self.due_date.map(|x| writeln!(f, "Due To: {}", x));
        self.description
            .as_ref()
            .map(|x| writeln!(f, "Description: {}", x));
        self.file
            .as_ref()
            .map(|x| writeln!(f, "File: {}", x.to_str().unwrap()));
        self.url.as_ref().map(|x| writeln!(f, "Url: {}", x));
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
}
