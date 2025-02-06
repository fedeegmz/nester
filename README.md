# Nester CLI

**Nester** is a command-line interface (CLI) tool written in Rust that helps generate basic structures for **Kotlin/Ktor** projects using **Koin** as a dependency injector.

## 🚀 Installation

### 📦 From Source (Git)
```bash
git clone https://github.com/fedeegmz/nester.git
cd nester
cargo build --release
sudo mv target/release/nester /usr/local/bin/
```

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
Proyecto
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

## 📋 Requirements

- **Rust 1.70+**
- An existing **Kotlin/Ktor** project

## 📜 License

This project is licensed under the GPL-3.0 License.
See the full license [here](LICENSE).
