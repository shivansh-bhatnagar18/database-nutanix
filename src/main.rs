mod store;
mod snapshot;

use clap::{Parser, Subcommand};
use store::KVStore;

#[derive(Parser)]
#[command(name = "kvstore")]
#[command(about = "A simple key-value store with snapshot support", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the database directory
    #[arg(short, long, default_value = "snapshots")] 
    db_dir: String,
}

#[derive(Subcommand)]
enum Commands {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
    Restore {
        hash: String,
    },
    ListSnapshots,
}

fn main() {
    let cli = Cli::parse();
    let mut store = KVStore::new();

    // Try to restore latest state
    let _ = store.restore_latest(&cli.db_dir);

    match &cli.command {
        Commands::Set { key, value } => {
            store.set(key.clone(), value.clone());
            println!("Set {} = {}", key, value);
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
    }

    if let Commands::Set { .. } | Commands::Delete { .. } = &cli.command {
        let hash = store.snapshot_hashed(&cli.db_dir).expect("Auto-snapshot failed");
        println!("Snapshot saved with hash: {}", hash);
    }
}