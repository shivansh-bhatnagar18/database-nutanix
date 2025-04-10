# 🚀 KVStore: Key-Value Store with Snapshot & Restore

KVStore is a simple and scalable key-value database built in Rust. It supports standard operations like `set`, `get`, and `delete`, along with automatic **snapshotting** of the database state. You can **restore** to any previous state using a short snapshot hash and **compare** differences between snapshots.

---

## 🔧 Setup

1. Clone the repo:
   ```bash
   git clone <your-repo-url>
   cd <your-repo-name>

2. Run with Cargo:

    cargo build

💡 Available Commands

# Set a key-value pair (creates snapshot)
cargo run -- set key value

# Get the value of a key
cargo run -- get key

# Delete a key (creates snapshot)
cargo run -- delete key

# List all snapshots
cargo run -- list-snapshots

# Restore to a previous snapshot by hash
cargo run -- restore <short_hash>

# Compare two snapshots
cargo run -- compare <hash1> <hash2>

📝 Snapshot files are saved inside the snapshots/ folder as .db files with operation type, key, and timestamp.
👀 Example

cargo run -- set language Rust
cargo run -- get language
cargo run -- delete language
cargo run -- list-snapshots
cargo run -- restore abc123ef
cargo run -- compare abc123ef def456ab

