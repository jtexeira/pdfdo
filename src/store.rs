pub mod cat;
pub mod task;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct StoreMap<K, T>
where
    K: Eq + std::hash::Hash,
{
    //Maybe move to hashmap later
    curr_id: K,
    map: HashMap<K, T>,
}

impl<K, T> Default for StoreMap<K, T>
where
    K: Default + Eq + std::hash::Hash,
{
    fn default() -> Self {
        StoreMap {
            curr_id: Default::default(),
            map: HashMap::new(),
        }
    }
}

impl<K, T> Display for StoreMap<K, T>
where
    T: Display,
    K: Display + Default + Eq + std::hash::Hash,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map
            .iter()
            .try_for_each(|(x, y)| writeln!(f, "{}: {}", x, y))?;
        Ok(())
    }
}
impl<'a, T, K> StoreMap<K, T>
where
    K: Copy + Eq + std::hash::Hash + std::ops::AddAssign<usize>,
{
    pub fn add_us(&mut self, obj: T) {
        while self.map.contains_key(&self.curr_id) {
            self.curr_id += 1;
        }
        self.map.insert(self.curr_id, obj);
    }
}

impl<'a, T, K> StoreMap<K, T>
where
    T: 'a + Serialize + Deserialize<'a>,
    K: 'a + Serialize + Deserialize<'a> + Eq + std::hash::Hash,
{
    pub fn add(&mut self, key: K, obj: T) {
        self.map.insert(key, obj);
    }

    pub fn rm(&mut self, id: &K) {
        self.map.remove(id);
    }

    pub fn get(&self, id: &K) -> Option<&T> {
        self.map.get(id)
    }

    pub fn save(&self, path: &PathBuf) -> std::io::Result<()> {
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
