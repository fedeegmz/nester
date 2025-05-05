#!/bin/bash

# Backup file pattern
BACKUP_PATTERN="/usr/local/bin/nester.backup_*"

# Find and delete backup files
echo "Searching for backup files..."
if ls $BACKUP_PATTERN 1> /dev/null 2>&1; then
    echo "Deleting backup files..."
    sudo rm -f $BACKUP_PATTERN
    echo "All backups have been removed."
else
    echo "No backup files found."
fi
