# Cyrus (Sirius) - All-in-One Language Management Tool v0.3.0

**Author**: Omid Nateghi  
**Engine**: Omid Coder  
**Language**: Rust 🦀  
**Version**: 0.3.0

Cyrus is a comprehensive, modular tool for managing programming language environments with local project isolation, global language installation capabilities, intelligent command aliasing, advanced templating, plugin system, workspace management, and enterprise-grade features.

## 🚀 Features

### Core Features
- **Multi-language Support**: Python, JavaScript/Node.js, Go, Rust, Java, PHP, Ruby (extensible)
- **Local Project Isolation**: Each project runs in its own environment
- **Global Language Management**: Install languages to `~/.cyrus`
- **Smart Command Aliasing**: Intelligent package manager integration
- **Fast & Lightweight**: Built with Rust for maximum performance
- **Cross-platform**: Windows, macOS, Linux support

### Advanced Features
- **Project Templates**: 10+ built-in templates with custom template support
- **Plugin System**: Extensible architecture with dynamic plugin loading
- **Workspace Management**: Multi-project development with dependency management
- **Configuration Profiles**: Environment-specific configurations (enterprise, performance, minimal, student)
- **Performance Optimization**: Parallel operations, caching, and benchmarking
- **Security Features**: Dependency auditing, signature verification, sandboxing
- **Enhanced Error Handling**: Smart recovery suggestions and detailed diagnostics

## 📦 Installation

### From Source
```bash
git clone https://github.com/omidnateghi/cyrus
cd cyrus
cargo build --release
./scripts/install.sh
```

### Binary Releases
Download from [GitHub Releases](https://github.com/omidnateghi/cyrus/releases)

## 🎯 Quick Start

### 1. Global Language Installation
```bash
# Install languages globally
cyrus install python3.11
cyrus install node20
cyrus install go1.21
cyrus install rust1.75
cyrus install java21

# List installed languages
cyrus list

# Show all supported languages
cyrus languages
```

### 2. Project Creation from Templates
```bash
# Create React TypeScript project
cyrus new react-typescript my-web-app

# Create Rust CLI tool
cyrus new rust-cli my-cli-tool

# Create Python API
cyrus new python-api my-api

# List available templates
cyrus template list

# Search templates
cyrus template search "web"
```

### 3. Project Management
```bash
# Initialize existing project
cyrus init

# Run commands with smart aliasing
cyrus run dev        # Auto-detects package manager
cyrus run test       # Runs tests
cyrus run build      # Builds the project

# Manage aliases
cyrus alias list     # Show all aliases
cyrus alias add t "npm test"  # Add custom alias
cyrus alias toggle   # Enable/disable aliasing
```

### 4. Workspace Management
```bash
# Create workspace
cyrus workspace init my-workspace

# Add projects to workspace
cyrus workspace add frontend ./frontend --language javascript --create
cyrus workspace add backend ./backend --language python --create
cyrus workspace add shared ./shared --language typescript --create

# Run commands across workspace
cyrus workspace run build --parallel
cyrus workspace run test
cyrus workspace status
```

## 📚 Commands Reference

### Core Commands
| Command | Description |
|---------|-------------|
| `cyrus install <lang><ver>` | Install language globally |
| `cyrus init` | Initialize new project with enhanced options |
| `cyrus new <template> <name>` | Create project from template |
| `cyrus run <command>` | Run command with smart aliasing |
| `cyrus list` | List installed languages |
| `cyrus languages` | Show supported languages |

### Template Commands
| Command | Description |
|---------|-------------|
| `cyrus template list` | List available templates |
| `cyrus template search <query>` | Search templates |
| `cyrus template show <name>` | Show template details |
| `cyrus template install <source>` | Install custom template |

### Plugin Commands
| Command | Description |
|---------|-------------|
| `cyrus plugin list` | List installed plugins |
| `cyrus plugin install <source>` | Install plugin |
| `cyrus plugin enable <name>` | Enable plugin |
| `cyrus plugin exec <plugin> <cmd>` | Execute plugin command |

### Workspace Commands
| Command | Description |
|---------|-------------|
| `cyrus workspace init <name>` | Create new workspace |
| `cyrus workspace add <name> <path>` | Add project to workspace |
| `cyrus workspace run <cmd>` | Run command in workspace |
| `cyrus workspace build --parallel` | Build all projects |
| `cyrus workspace status` | Show workspace status |

### Profile Commands
| Command | Description |
|---------|-------------|
| `cyrus profile list` | List available profiles |
| `cyrus profile switch <name>` | Switch active profile |
| `cyrus profile create <name>` | Create new profile |
| `cyrus profile export <name>` | Export profile |

## 🌍 Supported Languages

| Language | Aliases | Package Managers | Templates |
|----------|---------|------------------|-----------|
| **Python** | py, python3 | pip, poetry, pipenv | python-api, python-cli, python-ml |
| **JavaScript/Node.js** | js, node, nodejs | npm, yarn, pnpm, bun | react-typescript, node-express, vue-typescript |
| **Rust** | rs | cargo | rust-cli, rust-web |
| **Go** | go | go mod | go-api |
| **Java** | java | maven, gradle | java-spring |
| **PHP** | php | composer | - |
| **Ruby** | rb | gem, bundler | - |

## 📄 Enhanced Project Configuration

### Basic Configuration (`cyrus.toml`)
```toml
name = "my-project"
language = "javascript"
version = "20"
package_manager = "bun"
enable_aliases = true

[scripts]
start = "node dist/index.js"
build = "tsc"
dev = "nodemon src/index.ts"

[custom_aliases]
t = "bun test"
b = "bun run build"
d = "bun run dev"

[environment]
NODE_ENV = "development"
PORT = "3000"
```

### Workspace Configuration (`cyrus-workspace.toml`)
```toml
name = "my-workspace"
description = "Full-stack application workspace"

[[members]]
name = "frontend"
path = "./frontend"
language = "javascript"
enabled = true

[[members]]
name = "backend"
path = "./backend"
language = "python"
enabled = true
dependencies = ["shared"]

[shared_config]
build_parallel = true
max_parallel_jobs = 4

[scripts.build]
command = "cyrus run build"
run_parallel = true

[scripts.test]
command = "cyrus run test"
run_parallel = false
continue_on_error = true
```

## 🔌 Plugin System

### Built-in Plugins
- **Docker Plugin**: Add Docker support to projects
- **Example Plugin**: Demonstration plugin

### Creating Custom Plugins
```rust
use cyrus::plugins::interface::*;

pub struct MyPlugin;

impl CyrusPlugin for MyPlugin {
    fn get_info(&self) -> PluginInfo {
        PluginInfo {
            name: "my-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "My custom plugin".to_string(),
            // ...
        }
    }
    
    async fn execute_command(&self, command: &str, args: &[String]) -> Result<()> {
        match command {
            "hello" => println!("Hello from my plugin!"),
            _ => anyhow::bail!("Unknown command: {}", command),
        }
        Ok(())
    }
    
    // ... implement other methods
}

cyrus_plugin!(MyPlugin);
```

### Plugin Usage
```bash
# Install plugin
cyrus plugin install git://github.com/user/my-plugin

# Use plugin
cyrus plugin exec my-plugin hello
```

## 🏢 Configuration Profiles

### Enterprise Profile
```bash
cyrus profile switch enterprise
# - Strict quality gates
# - Required tests and linting  
# - Security auditing enabled
# - Stable package managers (pnpm, poetry)
```

### Performance Profile
```bash
cyrus profile switch performance  
# - Fastest package managers (bun, uv)
# - Parallel operations enabled
# - Optimized settings
```

### Student Profile
```bash
cyrus profile switch student
# - Beginner-friendly tools
# - Educational aliases
# - Simplified configuration
```

## 🔒 Security Features

```bash
# Audit dependencies
cyrus security audit

# Verify installations
cyrus security verify

# Show security status
cyrus security status

# Update security database
cyrus security update
```

## ⚡ Performance Features

```bash
# Show performance metrics
cyrus perf metrics

# Optimize configuration
cyrus perf optimize

# Benchmark operations
cyrus perf benchmark install

# Parallel downloads (4 concurrent by default)
cyrus install --parallel python3.11 node20 go1.21
```

## 🛠️ Development Tools

```bash
# Debug information
cyrus dev debug

# Validate configuration
cyrus dev validate

# Generate shell completions
cyrus dev completions bash > /etc/bash_completion.d/cyrus

# Environment information
cyrus dev env

# Clean caches
cyrus dev clean
```

## 🎭 Smart Aliasing Examples

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
cyrus run lint    # → poetry run flake8
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
cyrus run clean   # → mvn clean
```

## 📋 Available Templates

### Web Development
- **react-typescript**: React 18 + TypeScript + Modern tooling
- **vue-typescript**: Vue 3 + TypeScript + Composition API
- **node-express**: Express.js API with TypeScript

### Backend Development
- **python-api**: FastAPI with async support and OpenAPI docs
- **go-api**: Go REST API with modern architecture
- **java-spring**: Spring Boot application with best practices

### CLI Tools
- **rust-cli**: Rust CLI with Clap argument parsing
- **python-cli**: Python CLI with Click and rich output

### Specialized
- **python-ml**: Machine Learning project with Jupyter, pandas, scikit-learn
- **rust-web**: Rust web application with Axum framework

### Template Features
Each template includes:
- ✅ Language-specific best practices
- ✅ Development dependencies
- ✅ Testing setup
- ✅ Linting and formatting
- ✅ CI/CD configuration
- ✅ Docker support (optional)
- ✅ Documentation templates

## 🏗️ Workspace Examples

### Full-Stack Web Application
```bash
cyrus workspace init fullstack-app
cd fullstack-app

# Add frontend
cyrus workspace add frontend ./apps/frontend \
  --language javascript --create

# Add backend API  
cyrus workspace add api ./apps/api \
  --language python --create

# Add shared library
cyrus workspace add shared ./libs/shared \
  --language typescript --create

# Set up dependencies
# (frontend depends on shared, api is independent)

# Build everything
cyrus workspace build --parallel

# Run development servers
cyrus workspace run dev --parallel --members frontend,api
```

### Microservices Architecture
```bash
cyrus workspace init microservices
cd microservices

# Add services
cyrus workspace add user-service ./services/user \
  --language java --create
cyrus workspace add order-service ./services/order \
  --language go --create  
cyrus workspace add notification-service ./services/notification \
  --language python --create
cyrus workspace add gateway ./gateway \
  --language javascript --create

# Add shared components
cyrus workspace add shared-proto ./shared/proto \
  --language proto --create

# Build in dependency order
cyrus workspace build

# Test all services
cyrus workspace test --parallel
```

## 🔧 Advanced Configuration

### Global Configuration (`~/.config/cyrus/config.toml`)
```toml
default_profile = "performance"

[security_settings]
verify_downloads = true
check_signatures = true
audit_dependencies = true
security_level = "Medium"

[network_settings]
timeout_seconds = 30
max_retries = 3
parallel_downloads = 6

[ui_settings]
colored_output = true
show_progress = true
verbosity = "Normal"
theme = "Dark"

[cache_settings]
enabled = true
max_size_mb = 2048
ttl_hours = 48
auto_cleanup = true

[plugin_settings]
enabled = true
auto_update_plugins = false

[global_aliases]
update-all = "update && plugin update && security update"
fresh-install = "dev clean && install"
check-all = "security audit && dev validate"
```

### Project Templates with Variables
```toml
# Template with custom variables
name = "{{project_name}}"
language = "{{language}}"
author = "{{author}}"
license = "{{license}}"

[variables.author]
description = "Project author name"
required = true
type = "String"

[variables.license]  
description = "Project license"
default_value = "MIT"
type = "Choice"
choices = ["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause"]

[variables.database]
description = "Database type"
default_value = "postgresql"
type = "Choice" 
choices = ["postgresql", "mysql", "sqlite", "mongodb"]
```

## 🚦 Error Handling & Recovery

Cyrus provides intelligent error handling with automated recovery suggestions:

```bash
# Example error with suggestions
$ cyrus run test
❌ Language 'python' version '3.11' is not installed

💡 Try:
  • Run 'cyrus install python3.11'
  • Run 'cyrus list' to see installed languages

# Network error with retry logic
$ cyrus install node20
🌐 Network error: Connection timeout
🔄 Retrying in 2 seconds... (1/3)
✅ Node.js 20 installed successfully
```

## 📊 Performance Metrics

```bash
$ cyrus perf metrics
📊 Performance Metrics:
  Cache hit rate: 92%
  Average download speed: 8.5 MB/s
  Language startup time: 120ms
  Command execution time: 35ms
  Plugin load time: 15ms
  Template generation time: 250ms

  Memory usage: 45MB
  Disk cache size: 1.2GB
  Active languages: 5
  Active plugins: 3
  Active workspaces: 2
```

## 🧪 Testing and Quality

### Built-in Quality Gates
```bash
# Configure quality requirements
cyrus profile create strict --base enterprise

# Set minimum standards
[quality_gates]
require_tests = true
min_test_coverage = 80.0
require_linting = true
require_security_audit = true
max_dependencies = 30
banned_dependencies = ["lodash", "moment"]

# Automatic validation
cyrus run test   # Fails if coverage < 80%
cyrus run lint   # Fails if linting errors
cyrus run audit  # Fails if vulnerabilities found
```

### Testing Framework Integration
```bash
# Language-specific test commands
cyrus run test        # Auto-detects test framework
  # Python: pytest
  # JavaScript: jest/vitest
  # Rust: cargo test
  # Go: go test
  # Java: mvn test

# Coverage reporting
cyrus run coverage    # Generate coverage reports
cyrus run coverage --html  # HTML coverage report
```

## 🔄 CI/CD Integration

### GitHub Actions Example
```yaml
name: Cyrus CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Cyrus
      run: |
        curl -sSL https://install.cyrus-lang.org | sh
        echo "$HOME/.cyrus/bin" >> $GITHUB_PATH
    
    - name: Setup project
      run: cyrus install --from-lock
    
    - name: Run tests
      run: cyrus run test
    
    - name: Security audit
      run: cyrus security audit
    
    - name: Build
      run: cyrus run build
```

### Docker Integration
```dockerfile
FROM rust:1.75 as cyrus-installer
RUN curl -sSL https://install.cyrus-lang.org | sh

FROM ubuntu:22.04
COPY --from=cyrus-installer /root/.cyrus /usr/local/cyrus
ENV PATH="/usr/local/cyrus/bin:$PATH"

WORKDIR /app
COPY cyrus.toml .
RUN cyrus install --from-lock

COPY . .
RUN cyrus run build
CMD ["cyrus", "run", "start"]
```

## 🌐 Multi-language Project Example

### Full-Stack Application Structure
```
my-fullstack-app/
├── cyrus-workspace.toml          # Workspace config
├── frontend/                     # React TypeScript
│   ├── cyrus.toml
│   ├── package.json
│   └── src/
├── backend/                      # Python FastAPI
│   ├── cyrus.toml  
│   ├── requirements.txt
│   └── src/
├── mobile/                       # React Native
│   ├── cyrus.toml
│   ├── package.json
│   └── src/
├── shared/                       # TypeScript library
│   ├── cyrus.toml
│   ├── package.json
│   └── src/
└── scripts/                      # Build/deploy scripts
    ├── deploy.sh
    └── test-all.sh
```

### Workspace Commands
```bash
# Development workflow
cyrus workspace run install        # Install all dependencies
cyrus workspace run dev --parallel # Start all dev servers
cyrus workspace run test          # Run all tests
cyrus workspace run build         # Build all projects
cyrus workspace run lint          # Lint all code

# Production workflow  
cyrus workspace run build --parallel
cyrus workspace run test --parallel
cyrus security audit
cyrus workspace run deploy
```

## 🎓 Learning Resources

### Interactive Tutorials
```bash
# Built-in tutorials
cyrus tutorial basics     # Cyrus fundamentals
cyrus tutorial templates  # Working with templates
cyrus tutorial workspace  # Multi-project development
cyrus tutorial plugins    # Extending with plugins

# Example projects
cyrus example create todo-app
cyrus example create microservice
cyrus example create cli-tool
```

### Community Templates
```bash
# Explore community templates
cyrus template search community
cyrus template install github:username/my-template
cyrus template list --source community
```

## 🤝 Contributing

### Development Setup
```bash
git clone https://github.com/omidnateghi/cyrus
cd cyrus

# Install development dependencies
rustup component add clippy rustfmt
cargo install cargo-watch

# Run tests
cargo test

# Development workflow
cargo watch -x test -x clippy -x fmt
```

### Plugin Development
```bash
# Create plugin template
cyrus template create my-plugin --type plugin

# Test plugin locally
cyrus plugin install ./my-plugin

# Publish plugin
cyrus plugin publish my-plugin
```

### Template Contribution
```bash
# Create template
cyrus template create my-template

# Test template
cyrus new my-template test-project

# Submit to registry
cyrus template submit my-template
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Omid Nateghi**
- Engine: Omid Coder
- Built with ❤️ and Rust 🦀
- GitHub: [@omidnateghi](https://github.com/omidnateghi)

## 🙏 Acknowledgments

- Rust community for excellent tooling
- All language communities for inspiration
- Contributors and plugin developers
- Users providing feedback and suggestions

## 📈 Roadmap

### v0.4.0 (Next Release)
- [ ] AI-powered code generation
- [ ] Language Server Protocol support
- [ ] Advanced dependency resolution
- [ ] Cloud workspace sync
- [ ] WebAssembly plugin support

### v0.5.0 (Future)
- [ ] Visual project builder
- [ ] Integrated debugging tools
- [ ] Performance profiling
- [ ] Team collaboration features
- [ ] Enterprise SSO integration

---

*Cyrus - Making language management simple, efficient, and intelligent for developers worldwide.*

**Download now and transform your development workflow!** 🚀
