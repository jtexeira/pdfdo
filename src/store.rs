pub mod cat;
pub mod task;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::BufWriter;

#[derive(Serialize, Deserialize)]
pub struct StoreMap<T> {
    //Maybe move to hashmap later
    curr_id: usize,
    map: HashMap<usize, T>,
}

impl<T> Default for StoreMap<T> {
    fn default() -> Self {
        StoreMap {
            curr_id: 0,
            map: HashMap::new(),
        }
    }
}

impl<T> Display for StoreMap<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map.iter().for_each(|(x, y)| {
            writeln!(f, "{}: {}", x, y);
        });
        Ok(())
    }
}

impl<'a, T> StoreMap<T>
where
    T: 'a + Serialize + Deserialize<'a>,
{
    pub fn add(&mut self, obj: T) {
        while self.map.contains_key(&self.curr_id) {
            self.curr_id += 1;
        }
        self.map.insert(self.curr_id, obj);
    }

    pub fn rm(&mut self, id: usize) {
        self.map.remove(&id);
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }
}
