#!/bin/bash

echo "🔧 Starting KVStore Demo..."

echo "➕ Setting keys..."
cargo run -- set name Alice
cargo run -- set role Developer

echo "🔍 Getting 'name'..."
cargo run -- get name

echo "❌ Deleting 'role'..."
cargo run -- delete role

echo "📂 Listing available snapshots..."
cargo run -- list-snapshots

# You can manually check the snapshot name from the output and insert it here
echo "🔄 Restoring a snapshot (update the hash before running)..."
# Replace with an actual hash printed above
RESTORE_HASH="your_snapshot_hash_here"
cargo run -- restore $RESTORE_HASH

echo "📊 Comparing two snapshots (update with real hashes)..."
# Replace with two actual snapshot hashes printed above
HASH1="hash1_here"
HASH2="hash2_here"
cargo run -- compare $HASH1 $HASH2

echo "✅ Demo Complete."
