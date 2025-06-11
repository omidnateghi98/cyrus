# Cyrus (Sirius) - All-in-One Language Management Tool

**Author**: Omid Nateghi  
**Engine**: Omid Coder  
**Language**: Rust 🦀  
**Version**: 0.2.0

Cyrus is a comprehensive, modular tool for managing programming language environments with local project isolation, global language installation capabilities, and intelligent command aliasing.

## 🚀 Features

- **Multi-language Support**: Python, JavaScript/Node.js, Go, Rust, Java, PHP, Ruby (extensible)
- **Local Project Isolation**: Each project runs in its own environment
- **Global Language Management**: Install languages to `~/.cyrus`
- **Smart Command Aliasing**: Intelligent package manager integration
- **Package Manager Integration**: Support for pip, npm, yarn, poetry, bun, cargo, maven, composer, bundler, etc.
- **Fast & Lightweight**: Built with Rust for maximum performance
- **Modular Architecture**: Easy to extend with new languages
- **Cross-platform**: Windows, macOS, Linux support

## 🎯 Smart Aliasing

Cyrus includes intelligent command aliasing that automatically maps commands to your project's package manager:

```bash
# Traditional way:
bun run dev
npm run build
poetry run pytest

# With Cyrus smart aliasing:
cyrus run dev    # → bun run dev (if using bun)
cyrus run build  # → npm run build (if using npm)
cyrus run test   # → poetry run pytest (if using poetry)

# Even shorter with custom aliases:
cyrus run dev    # → Your custom alias
cyrus run t      # → test command
cyrus run b      # → build command
```

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
cyrus install rust1.75
cyrus install java21
cyrus install php8.3
cyrus install ruby3.3

# List installed languages
cyrus list

# Show all supported languages
cyrus languages
```

### Project Management
```bash
# Create and initialize new project
mkdir my-app && cd my-app
cyrus init

# Run commands with smart aliasing
cyrus run dev        # Runs the dev script with your package manager
cyrus run test       # Runs tests
cyrus run build      # Builds the project

# Manage aliases
cyrus alias list     # Show all aliases
cyrus alias add t "npm test"  # Add custom alias
cyrus alias remove t # Remove alias
cyrus alias toggle   # Enable/disable aliasing

# Show project configuration
cyrus config
```

## 📚 Commands

| Command | Description |
|---------|-------------|
| `cyrus install <lang><ver>` | Install language globally |
| `cyrus init` | Initialize new project with enhanced options |
| `cyrus run <command>` | Run command with smart aliasing |
| `cyrus alias <action>` | Manage project aliases |
| `cyrus languages` | Show supported languages |
| `cyrus list` | List installed languages |
| `cyrus config` | Show configuration |
| `cyrus update` | Update Cyrus or languages |
| `cyrus remove <lang><ver>` | Remove language |
| `cyrus version` | Show version info |

## 🌍 Supported Languages

| Language | Aliases | Package Managers | Default Version |
|----------|---------|------------------|-----------------|
| **Python** | py, python3 | pip, poetry, pipenv | 3.11 |
| **JavaScript/Node.js** | js, node, nodejs | npm, yarn, pnpm, bun | 20 |
| **Go** | go | go mod | 1.21 |
| **Rust** | rs | cargo | 1.75 |
| **Java** | java | maven, gradle | 21 |
| **PHP** | php | composer | 8.3 |
| **Ruby** | rb | gem, bundler | 3.3 |

## 📄 Enhanced Project Configuration

`cyrus.toml` with smart aliasing:
```toml
name = "my-project"
language = "javascript"
version = "20"
package_manager = "bun"
dependencies = ["express", "typescript"]
dev_dependencies = ["jest", "nodemon"]

# Enable smart aliasing
enable_aliases = true

[scripts]
start = "node dist/index.js"
build = "tsc"
dev = "nodemon src/index.ts"

# Custom aliases for shorter commands
[custom_aliases]
t = "bun test"
b = "bun run build"
d = "bun run dev"
install = "bun install"

[environment]
NODE_ENV = "development"
PORT = "3000"
```

## 🎭 Aliasing Examples

### JavaScript/Node.js with Bun
```bash
cyrus run dev     # → bun run dev
cyrus run test    # → bun test
cyrus run build   # → bun run build
cyrus run install # → bun install
```

### Python with Poetry
```bash
cyrus run test    # → poetry run pytest
cyrus run install # → poetry install
cyrus run shell   # → poetry shell
```

### Rust
```bash
cyrus run build   # → cargo build
cyrus run test    # → cargo test
cyrus run check   # → cargo check
cyrus run clippy  # → cargo clippy
```

### Java with Maven
```bash
cyrus run compile # → mvn compile
cyrus run test    # → mvn test
cyrus run package # → mvn package
```

## 🛠️ Development

```bash
# Run tests
cargo test

# Build release
cargo build --release

# Generate documentation
cargo doc --open

# Test with example projects
cd examples/enhanced
cyrus init
cyrus run dev
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

*Cyrus - Making language management simple, efficient, and intelligent for developers worldwide.*