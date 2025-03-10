#!/bin/bash

# Binary name
BINARY_NAME="nester"

# Path where the binary will be installed
INSTALL_PATH="/usr/local/bin/$BINARY_NAME"

echo "Building the project in release mode..."
cargo build --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Error: Build failed. Aborting."
    exit 1
fi

# Check if the binary already exists
if [ -f "$INSTALL_PATH" ]; then
    BACKUP_PATH="${INSTALL_PATH}.backup_$(date +%Y%m%d%H%M%S)"
    echo "Existing binary found. Creating backup at $BACKUP_PATH..."
    sudo mv "$INSTALL_PATH" "$BACKUP_PATH"
fi

# Move the new binary to /usr/local/bin
echo "Installing $BINARY_NAME to $INSTALL_PATH..."
sudo mv "target/release/$BINARY_NAME" "$INSTALL_PATH"

# Check if the binary was moved successfully
if [ $? -eq 0 ]; then
    echo "Installation complete. You can run '$BINARY_NAME' from anywhere."
else
    echo "Error: Failed to move the binary. Check permissions."
    exit 1
fi
