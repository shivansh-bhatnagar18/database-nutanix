use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use crate::snapshot;
use crate::store::trait_def::StorageBackend;

#[derive(Serialize, Deserialize, Default)]
pub struct KVStore {
    pub map: HashMap<String, String>,
}

impl StorageBackend for KVStore {
    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }

    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(self)?)
    }

    fn deserialize(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let restored: KVStore = bincode::deserialize(data)?;
        self.map = restored.map;
        Ok(())
    }
}

impl KVStore {
    pub fn new() -> Self {
        KVStore { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn snapshot_named(&self, dir: &str, operation: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        fs::create_dir_all(dir)?;
        let bytes = StorageBackend::serialize(self)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let full_hash = format!("{:x}", hasher.finalize());
        let short_hash = &full_hash[..8];
        let timestamp = Utc::now().format("%Y%m%dT%H%M%S");
        let filename = format!("{}_{}-{}_{}.db", timestamp, operation, key, short_hash);
        let file_path = format!("{}/{}", dir, filename);
        let mut file = File::create(&file_path)?;
        file.write_all(&bytes)?;
        Ok(filename)
    }

    pub fn restore_by_hash(&mut self, dir: &str, hash: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name.contains(hash) {
                    let restored = snapshot::load_snapshot(path.to_str().unwrap())?;
                    self.map = restored.map;
                    return Ok(())
                }
            }
        }
        Err(From::from("Snapshot hash not found"))
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
                    path.file_name().and_then(|s| s.to_str()).map(String::from)
                } else {
                    None
                }
            })
            .collect();
        Ok(entries)
    }

    pub fn compare_snapshots(dir: &str, hash1: &str, hash2: &str) -> Result<String, Box<dyn std::error::Error>> {
        fn find_snapshot_path(dir: &str, hash: &str) -> Option<String> {
            fs::read_dir(dir).ok()?.filter_map(Result::ok).find_map(|entry| {
                let path = entry.path();
                let name = path.file_name()?.to_str()?;
                if name.contains(hash) {
                    Some(path.to_string_lossy().to_string())
                } else {
                    None
                }
            })
        }

        let path1 = find_snapshot_path(dir, hash1).ok_or("First snapshot not found")?;
        let path2 = find_snapshot_path(dir, hash2).ok_or("Second snapshot not found")?;

        let snap1: KVStore = snapshot::load_snapshot(&path1)?;
        let snap2: KVStore = snapshot::load_snapshot(&path2)?;

        let mut report = String::new();
        let all_keys: HashSet<_> = snap1.map.keys().chain(snap2.map.keys()).collect();

        for key in all_keys {
            match (snap1.map.get(key), snap2.map.get(key)) {
                (Some(v1), Some(v2)) if v1 != v2 => {
                    report += &format!("🔁 Changed: {} => '{}' -> '{}'\n", key, v1, v2);
                }
                (Some(_), None) => {
                    report += &format!("❌ Removed: {}\n", key);
                }
                (None, Some(_)) => {
                    report += &format!("🆕 Added: {}\n", key);
                }
                _ => {}
            }
        }

        Ok(report)
    }
}