# Cyrus (Sirius) - All-in-One Language Management Tool

**Author**: Omid Nateghi  
**Engine**: Omid Coder  
**Language**: Rust 🦀  

Cyrus is a comprehensive, modular tool for managing programming language environments with local project isolation and global language installation capabilities.

## 🚀 Features

- **Multi-language Support**: Python, JavaScript/Node.js, Go (extensible)
- **Local Project Isolation**: Each project runs in its own environment
- **Global Language Management**: Install languages to `~/.cyrus`
- **Package Manager Integration**: Support for pip, npm, yarn, poetry, bun, etc.
- **Fast & Lightweight**: Built with Rust for maximum performance
- **Modular Architecture**: Easy to extend with new languages
- **Cross-platform**: Windows, macOS, Linux support

## 📦 Installation

1. **Build from source**:
   ```bash
   ./generate_project.sh
   cd cyrus
   cargo build --release
   ```

2. **Install globally**:
   ```bash
   ./scripts/install.sh
   ```

## 🎯 Quick Start

### Global Language Installation
```bash
# Install languages globally
cyrus install python3.11
cyrus install node20
cyrus install go1.21

# List installed languages
cyrus list
```

### Project Management
```bash
# Create and initialize new project
mkdir my-app && cd my-app
cyrus init

# Run commands in project environment
cyrus run python app.py
cyrus run npm start
cyrus run go build

# Show project configuration
cyrus config
```

## 📚 Commands

| Command | Description |
|---------|-------------|
| `cyrus install <lang><ver>` | Install language globally |
| `cyrus init` | Initialize new project |
| `cyrus run <command>` | Run command in project env |
| `cyrus list` | List installed languages |
| `cyrus config` | Show configuration |
| `cyrus update` | Update Cyrus or languages |
| `cyrus remove <lang><ver>` | Remove language |
| `cyrus version` | Show version info |

## 🏗️ Architecture

```
cyrus/
├── src/
│   ├── core/           # Core functionality
│   ├── languages/      # Language handlers
│   ├── commands/       # CLI commands
│   ├── installer/      # Installation logic
│   ├── runtime/        # Runtime environment
│   └── utils/          # Utilities
├── config/             # Language configurations
├── docs/               # Multi-language documentation
├── tests/              # Unit & integration tests
└── examples/           # Example projects
```

## 🌍 Supported Languages

- **Python**: 3.8, 3.9, 3.10, 3.11, 3.12
- **JavaScript/Node.js**: 16, 18, 20, 21
- **Go**: 1.19, 1.20, 1.21, 1.22

## 📄 Project Configuration

`cyrus.toml` example:
```toml
name = "my-project"
language = "python"
version = "3.11"
package_manager = "pip"
dependencies = ["requests", "flask"]
dev_dependencies = ["pytest", "black"]

[scripts]
start = "python app.py"
test = "pytest"
lint = "black ."

[environment]
DEBUG = "true"
PORT = "8000"
```

## 🛠️ Development

```bash
# Run tests
cargo test

# Build release
cargo build --release

# Generate documentation
cargo doc --open
```

## 📖 Documentation

- [English](docs/en/README.md)
- [فارسی](docs/fa/README.md)
- [العربية](docs/ar/README.md)
- [中文](docs/zh/README.md)
- [Español](docs/es/README.md)

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Omid Nateghi**
- Engine: Omid Coder
- Built with ❤️ and Rust 🦀

---

*Cyrus - Making language management simple and efficient for developers worldwide.*
