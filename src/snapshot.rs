use std::fs::File;
use std::io::{BufReader, BufWriter};
use bincode;
use crate::store::KVStore;

pub fn save_snapshot(store: &KVStore, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, store)?;
    Ok(())
}

pub fn load_snapshot(path: &str) -> Result<KVStore, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let store: KVStore = bincode::deserialize_from(reader)?;
    Ok(store)
}