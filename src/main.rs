mod store;
mod snapshot;
mod cli;
mod metadata;

use clap::Parser;
use cli::{Cli, Commands};
use store::kv::KVStore;
// use store::trait_def::StorageBackend;

fn main() {
    let cli = Cli::parse();
    let mut store = KVStore::new();

    // Try to restore latest state
    if !matches!(cli.command, Commands::Restore { .. }) {
        let _ = store.restore_latest(&cli.db_dir);
    }

        match &cli.command {
            Commands::Set { key, value } => {
                store.set(key.clone(), value.clone());
                println!("Set {} = {}", key, value);
                let snap_id = store.snapshot_named(&cli.db_dir, "set", key).expect("Auto-snapshot failed");
                println!("Snapshot saved: {}", snap_id);
            }
            Commands::Get { key } => {
                match store.get(key) {
                    Some(value) => println!("{}", value),
                    None => println!("Key not found"),
                }
            }
            Commands::Delete { key } => {
                store.delete(key);
                println!("Deleted key: {}", key);
                let snap_id = store.snapshot_named(&cli.db_dir, "delete", key).expect("Auto-snapshot failed");
                println!("Snapshot saved: {}", snap_id);
            }
            Commands::Restore { hash } => {
                store.restore_by_hash(&cli.db_dir, hash).expect("Restore failed");
                println!("Restored to snapshot {}", hash);
            }
            Commands::ListSnapshots => {
                let snapshots = store.list_snapshots(&cli.db_dir).unwrap_or_default();
                for snap in snapshots {
                    println!("{}", snap);
                }
            }
            Commands::Compare { hash1, hash2 } => {
                match KVStore::compare_snapshots(&cli.db_dir, hash1, hash2) {
                    Ok(diff) => println!("{}", diff),
                    Err(e) => println!("Failed to compare: {}", e),
                }
            }
        }
    }