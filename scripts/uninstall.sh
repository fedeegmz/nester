#!/bin/bash

# Binary name
BINARY_NAME="nester"

# Installation path
INSTALL_PATH="/usr/local/bin/$BINARY_NAME"

# Check if the binary exists
if [ -f "$INSTALL_PATH" ]; then
    echo "Removing $BINARY_NAME from $INSTALL_PATH..."
    sudo rm "$INSTALL_PATH"
    
    if [ $? -eq 0 ]; then
        echo "$BINARY_NAME has been successfully uninstalled."
    else
        echo "Error: Failed to remove $BINARY_NAME."
        exit 1
    fi
else
    echo "Error: $BINARY_NAME is not installed."
    exit 1
fi
