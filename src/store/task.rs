use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub due_date: Option<NaiveDate>,
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
        write!(f, "{}", self.short_print())?;
        Ok(())
    }
}

impl Task {
    pub fn new(
        due_date: Option<NaiveDate>,
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
        if let Some(x) = self.cat.as_ref() {
            string.push_str(format!("[{}] ", x).as_str());
        }
        string.push_str(format!("{}: ", self.name).as_str());
        if let Some(x) = self.due_date.as_ref() {
            string.push_str(format!("-{}-", x).as_str());
        }
        string
    }
    pub fn long_print(&self) -> String {
        let mut string = format!("{}\n", self.name);
        if let Some(x) = self.cat.as_ref() {
            string.push_str(format!("Category: {}\n", x).as_str());
        }
        if let Some(x) = self.due_date {
            string.push_str(format!("Due To: {}\n", x).as_str());
        }
        if let Some(x) = self.description.as_ref() {
            string.push_str(format!("Description: {}\n", x).as_str());
        }
        if let Some(x) = self.file.as_ref() {
            string.push_str(format!("File: {}\n", x.to_str().unwrap()).as_str());
        }
        if let Some(x) = self.url.as_ref() {
            string.push_str(format!("Url: {}\n", x).as_str());
        }
        string
    }
    pub fn get_work_dir(&self) -> Option<PathBuf> {
        if let Some(s) = &self.file {
            let mut z = s.clone();
            z.pop();
            return Some(z);
        }
        None
    }
    pub fn update(
        &mut self,
        due_date: Option<NaiveDate>,
        file: Option<PathBuf>,
        name: Option<String>,
        description: Option<String>,
        url: Option<Url>,
        cat: Option<String>,
    ) {
        if due_date.is_some() {
            self.due_date = due_date
        };
        if file.is_some() {
            self.file = file
        };
        if let Some(d) = name {
            self.name = d
        };
        if description.is_some() {
            self.description = description
        };
        if url.is_some() {
            self.url = url
        };
        if cat.is_some() {
            self.cat = cat
        };
    }
}
