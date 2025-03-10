# Nester CLI

**Nester** is a command-line interface (CLI) tool written in Rust that helps generate basic structures for **Kotlin/Ktor** projects using **Koin** as a dependency injector.

## 🚀 Installation

### 🔧 Install Using Script

```bash
chmod +x ./scripts/install.sh
./scripts/install.sh
```

This script:
1. **Compiles** the project in release mode.
2. **Checks if a previous version exists** in `/usr/local/bin/`.
   - If found, it creates a **backup** before replacing it.
3. **Moves the new binary** to `/usr/local/bin/`.

### 🏹 Arch Linux (AUR)

```bash
yay -S nester
```

### 📁 Precompiled Binary

```bash
wget https://github.com/fedeegmz/nester/releases/download/v0.1.0/nester
chmod +x nester
sudo mv nester /usr/local/bin/
```

## 🛠 Usage

### ✅ Generate a New Module

```bash
nester -g module -n users
```

This command generates a new module named users inside your Kotlin/Ktor project.

```
Project
.
└── root-dir
    └── src
        └── main
            └── kotlin
                ├── users
                │   ├── Injection.kt
                │   ├── Routing.kt
                │   └── Service.kt
                └── Application.kt
```

## ❌ Uninstallation

### 🔥 Remove Nester

To uninstall `nester`, run:

```bash
chmod +x ./scripts/uninstall.sh
./scripts/uninstall.sh
```

This script:
1. **Checks if `nester` exists** in `/usr/local/bin/`.
2. **Removes the binary** if found.

### 🧹 Clean Backup Files

If you want to remove all backup files created during installation, run:

```bash
chmod +x ./scripts/clean_backups.sh
./scripts/clean_backups.sh
```

This script:
1. **Finds all backup files** matching `/usr/local/bin/nester.backup_*`.
2. **Deletes them permanently**.

## 📋 Requirements

- **Rust 1.70+**
- An existing **Kotlin/Ktor** project

## 📜 License

This project is licensed under the GPL-3.0 License.
See the full license [here](LICENSE).
