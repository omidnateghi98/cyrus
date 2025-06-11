#!/bin/bash

echo "🔨 Building Cyrus..."

# Build for release
cargo build --release

# Create distribution directory
mkdir -p dist

# Copy binary
cp target/release/cyrus dist/

# Copy configuration files
cp -r config dist/

echo "✅ Build completed! Binary available at dist/cyrus"
