use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::snapshot;

#[derive(Serialize, Deserialize, Default)]
pub struct KVStore {
    pub map: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Self {
        KVStore { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn snapshot_hashed(&self, dir: &str) -> Result<String, Box<dyn std::error::Error>> {
        fs::create_dir_all(dir)?;
        let bytes = bincode::serialize(&self)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = format!("{:x}", hasher.finalize());
        let file_path = format!("{}/{}.db", dir, hash);
        let mut file = File::create(&file_path)?;
        file.write_all(&bytes)?;
        Ok(hash)
    }

    pub fn restore_by_hash(&mut self, dir: &str, hash: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.db", dir, hash);
        let restored: KVStore = snapshot::load_snapshot(&file_path)?;
        self.map = restored.map;
        Ok(())
    }

    pub fn restore_latest(&mut self, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries: Vec<_> = fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("db"))
            .collect();

        entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
        if let Some(entry) = entries.last() {
            let path = entry.path();
            let restored: KVStore = snapshot::load_snapshot(path.to_str().unwrap())?;
            self.map = restored.map;
        }
        Ok(())
    }

    pub fn list_snapshots(&self, dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let entries: Vec<String> = fs::read_dir(dir)?
            .filter_map(Result::ok)
            .filter_map(|e| {
                let path = e.path();
                if path.extension().and_then(|s| s.to_str()) == Some("db") {
                    path.file_stem().and_then(|s| s.to_str()).map(String::from)
                } else {
                    None
                }
            })
            .collect();
        Ok(entries)
    }
}