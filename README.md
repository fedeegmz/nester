# Nester CLI

Nester is a command-line interface (CLI) tool written in Rust that facilitates generating basic structures for Kotlin/Ktor projects following the MVC design pattern (similar to NestJS).

## Installation

```bash
git clone https://github.com/fedeegmz/nester.git
cd nester

cargo build --release
sudo mv target/release/nester /usr/local/bin/
```

## Usage

### Generate a new module

```bash
nester -g module -n users
```

## Features

- MVC structure generation for Kotlin/Ktor
- Automatic generation of:
  - Modules
  - Services
  - Routing
  - (Coming soon) Entities
  - (Coming soon) DAOs

## Requirements

- Rust 1.70 or higher
- Existing Kotlin/Ktor project

## License

This project is licensed under the GPL-3.0 License.
