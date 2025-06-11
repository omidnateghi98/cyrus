# Cyrus - All-in-One Language Management Tool

Cyrus is a comprehensive language management tool that allows developers to install, manage, and run multiple programming language environments locally and globally.

## Features

- 🚀 **Multi-language support**: Python, JavaScript/Node.js, Go
- 🔧 **Local project isolation**: Each project has its own environment
- 📦 **Package manager integration**: Support for pip, npm, yarn, poetry, etc.
- 🌐 **Global language installation**: Install languages to ~/.cyrus
- ⚡ **Fast and lightweight**: Built with Rust for maximum performance
- 🛠️ **Modular architecture**: Easy to extend with new languages

## Installation

Download the latest release for your platform or build from source:

```bash
cargo build --release
```

## Quick Start

### Install a language globally
```bash
cyrus install python3.11
cyrus install node20
cyrus install go1.21
```

### Initialize a new project
```bash
mkdir my-project && cd my-project
cyrus init
```

### Run commands in project environment
```bash
cyrus run python app.py
cyrus run npm start
cyrus run go build
```

## Commands

- `cyrus install <language><version>` - Install a language globally
- `cyrus init` - Initialize a new project
- `cyrus run <command>` - Run commands in project environment
- `cyrus list` - List installed languages
- `cyrus config` - Show configuration
- `cyrus version` - Show version information

## Author

**Omid Nateghi** - Creator and maintainer
**Engine**: Omid Coder
