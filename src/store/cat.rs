use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Category {
    name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Url>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    work_dir: Option<PathBuf>,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        self.description.as_ref().map(|x| write!(f, ": {}", x));
        Ok(())
    }
}

impl Category {
    pub fn new(
        name: &str,
        description: Option<String>,
        url: Option<Url>,
        work_dir: Option<PathBuf>,
    ) -> Self {
        Category {
            name: name.to_owned(),
            description,
            url,
            work_dir: work_dir.map(|x| x.canonicalize().unwrap()),
        }
    }
    pub fn long_print(&self) -> String {
        let mut string = format!("{}\n", self.name);
        if let Some(x) = self.description.as_ref() {
            string.push_str(format!("Description: {}", x).as_str());
        }
        if let Some(x) = self.url.as_ref() {
            string.push_str(format!("Url: {}", x).as_str());
        }
        if let Some(x) = self.work_dir.as_ref() {
            string.push_str(format!("Work Directory: {}", x.to_str().unwrap()).as_str());
        }
        string
    }
}
