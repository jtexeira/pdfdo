use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Category {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_dir: Option<PathBuf>,
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
}
