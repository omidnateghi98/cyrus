# Cyrus - 一体化编程语言管理工具

Cyrus 是一个综合性的编程语言环境管理工具，支持本地项目隔离和全局语言安装功能。

## 特性

- 🚀 **多语言支持**: Python、JavaScript/Node.js、Go
- 🔧 **本地项目隔离**: 每个项目都有自己的环境
- 📦 **包管理器集成**: 支持 pip、npm、yarn、poetry 等
- 🌐 **全局语言管理**: 将语言安装到 ~/.cyrus
- ⚡ **快速轻量**: 使用 Rust 构建，性能最佳
- 🛠️ **模块化架构**: 易于扩展新语言

## 安装

下载适合您平台的最新版本或从源码构建：

```bash
cargo build --release
```

## 快速开始

### 全局安装语言
```bash
cyrus install python3.11
cyrus install node20
cyrus install go1.21
```

### 初始化新项目
```bash
mkdir my-project && cd my-project
cyrus init
```

### 在项目环境中运行命令
```bash
cyrus run python app.py
cyrus run npm start
cyrus run go build
```

## 作者

**Omid Nateghi** - 创建者和维护者
**引擎**: Omid Coder
