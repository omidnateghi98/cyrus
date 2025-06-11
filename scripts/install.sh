#!/bin/bash

echo "📦 Installing Cyrus..."

# Detect platform
case "$(uname -s)" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    CYGWIN*)    PLATFORM=windows;;
    MINGW*)     PLATFORM=windows;;
    *)          PLATFORM=unknown;;
esac

echo "Detected platform: $PLATFORM"

# Install binary to appropriate location
if [ "$PLATFORM" = "windows" ]; then
    INSTALL_DIR="$HOME/bin"
else
    INSTALL_DIR="/usr/local/bin"
fi

mkdir -p "$INSTALL_DIR"
cp dist/cyrus "$INSTALL_DIR/"

echo "✅ Cyrus installed to $INSTALL_DIR/cyrus"
echo "Make sure $INSTALL_DIR is in your PATH"
