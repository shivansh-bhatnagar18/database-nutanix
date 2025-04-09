use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kvstore")]
#[command(about = "A scalable key-value store with snapshot support", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the database directory
    #[arg(short, long, default_value = "snapshots")]
    pub db_dir: String,
}

#[derive(Subcommand)]
pub enum Commands {
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
    Compare {
        hash1: String,
        hash2: String,
    },
}