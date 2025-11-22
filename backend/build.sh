#!/bin/bash
set -e

echo "Building Rust backend for Heroku..."

# Install dependencies
cargo build --release

echo "Build complete!"
