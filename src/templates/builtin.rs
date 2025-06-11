// src/templates/builtin.rs
//! Built-in project templates with comprehensive configurations

use super::*;
use std::collections::HashMap;

pub fn create_react_typescript_template() -> ProjectTemplate {
    let mut files = HashMap::new();

    files.insert("src/App.tsx".to_string(), r#"import React from 'react';
import './App.css';

interface AppProps {}

const App: React.FC<AppProps> = () => {
  return (
    <div className="App">
      <header className="App-header">
        <h1>{{project_name}}</h1>
        <p>Built with Cyrus + React + TypeScript</p>
        <p>Created by {{author}}</p>
      </header>
    </div>
  );
};

export default App;
"#.to_string());

    files.insert("src/index.tsx".to_string(), r#"import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
"#.to_string());

    files.insert("src/App.css".to_string(), r#".App {
  text-align: center;
}

.App-header {
  background-color: #282c34;
  padding: 20px;
  color: white;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: calc(10px + 2vmin);
}

.App-header h1 {
  margin-bottom: 20px;
}

.App-header p {
  margin: 10px 0;
}
"#.to_string());

    files.insert("src/index.css".to_string(), r#"body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

code {
  font-family: source-code-pro, Menlo, Monaco, Consolas, 'Courier New',
    monospace;
}
"#.to_string());

    files.insert("public/index.html".to_string(), r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="#.parse().unwrap()000000" />
    <meta name="description" content="{{project_name}} - Built with Cyrus" />
    <title>{{project_name}}</title>
</head>
<body>
    <noscript>You need to enable JavaScript to run this app.</noscript>
    <div id="root"></div>
</body>
</html>
" # .to_string());

    files.insert("tsconfig.json".to_string(), r#"{
  "compilerOptions": {
    "target": "es5",
    "lib": ["dom", "dom.iterable", "es6"],
    "allowJs": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": [
    "src"
  ]
}
"#.to_string());

    files.insert("README.md".to_string(), r#"# {{project_name}}

A React TypeScript application built with Cyrus.

## Features

- ⚛️ React 18
- 🔷 TypeScript
- 🎨 CSS Modules support
- 🔧 Built with Cyrus for easy development

## Getting Started

1. Install dependencies:
   ```bash
   cyrus run install
   ```

2. Start the development server:
   ```bash
   cyrus run dev
   ```

3. Build for production:
   ```bash
   cyrus run build
   ```

## Author

Created by {{author}} in {{current_year}}.
"#.to_string());

    let mut scripts = HashMap::new();
    scripts.insert("start".to_string(), "react-scripts start".to_string());
    scripts.insert("build".to_string(), "react-scripts build".to_string());
    scripts.insert("test".to_string(), "react-scripts test".to_string());
    scripts.insert("eject".to_string(), "react-scripts eject".to_string());

    let mut aliases = HashMap::new();
    aliases.insert("dev".to_string(), "npm start".to_string());
    aliases.insert("serve".to_string(), "npm run build && npx serve -s build".to_string());

    // Features
    let mut features = Vec::new();

    // Router feature
    let mut router_files = HashMap::new();
    router_files.insert("src/components/Router.tsx".to_string(), r#"import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import App from '../App';

const Router: React.FC = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
      </Routes>
    </BrowserRouter>
  );
};

export default Router;
"#.to_string());

    features.push(TemplateFeature {
        name: "router".to_string(),
        description: "Add React Router for navigation".to_string(),
        enabled_by_default: false,
        dependencies: vec!["react-router-dom".to_string(), "@types/react-router-dom".to_string()],
        files: router_files,
        scripts: HashMap::new(),
        post_install_commands: vec![],
    });

    ProjectTemplate {
        name: "react-typescript".to_string(),
        description: "React application with TypeScript and modern tooling".to_string(),
        version: "1.0.0".to_string(),
        author: "Cyrus Templates".to_string(),
        language: "javascript".to_string(),
        language_version: "20".to_string(),
        package_manager: "npm".to_string(),
        metadata: TemplateMetadata {
            category: TemplateCategory::Web,
            tags: vec!["react".to_string(), "typescript".to_string(), "web".to_string()],
            difficulty: DifficultyLevel::Intermediate,
            min_cyrus_version: "0.3.0".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            documentation: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        files,
        dependencies: vec![
            "react".to_string(),
            "@types/react".to_string(),
            "react-dom".to_string(),
            "@types/react-dom".to_string(),
            "typescript".to_string(),
        ],
        dev_dependencies: vec![
            "@types/node".to_string(),
            "react-scripts".to_string(),
            "@testing-library/react".to_string(),
            "@testing-library/jest-dom".to_string(),
        ],
        scripts,
        aliases,
        environment: HashMap::new(),
        post_install_commands: vec![
            PostInstallCommand {
                command: "npm".to_string(),
                args: vec!["install".to_string()],
                working_directory: None,
                condition: None,
                ignore_failure: false,
            }
        ],
        variables: HashMap::new(),
        features,
        hooks: TemplateHooks::default(),
    }
}

pub fn create_rust_cli_template() -> ProjectTemplate {
    let mut files = HashMap::new();

    files.insert("src/main.rs".to_string(), r#"use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "{{project_name_snake}}")]
#[command(about = "{{project_name}} - A CLI tool built with Cyrus")]
#[command(author = "{{author}}")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello to someone
    Hello {
        /// Name of the person to greet
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Show detailed version information
    Version,
    /// Run in interactive mode
    Interactive,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            let name = name.unwrap_or_else(|| "World".to_string());
            println!("Hello, {}! 👋", name);
            println!("This is {{project_name}} built with Cyrus.");
        }
        Commands::Version => {
            println!("{{project_name}} v1.0.0");
            println!("Built with Rust and Cyrus");
            println!("Author: {{author}}");
        }
        Commands::Interactive => {
            println!("Welcome to {{project_name}} interactive mode!");
            // Add interactive functionality here
        }
    }

    Ok(())
}
"#.to_string());

    files.insert("Cargo.toml".to_string(), r#"[package]
name = "{{project_name_snake}}"
version = "0.1.0"
edition = "2021"
authors = ["{{author}}"]
description = "{{project_name}} - A CLI tool built with Cyrus"
license = "MIT"

[[bin]]
name = "{{project_name_snake}}"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
"#.to_string());

    files.insert("README.md".to_string(), r#"# {{project_name}}

A command-line tool built with Rust and Cyrus.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Say hello
{{project_name_snake}} hello --name "Your Name"

# Show version
{{project_name_snake}} version

# Interactive mode
{{project_name_snake}} interactive
```

## Development

Built with Cyrus for easy Rust development.

```bash
# Build
cyrus run build

# Run
cyrus run run

# Test
cyrus run test

# Check code
cyrus run check
```

## Author

Created by {{author}} in {{current_year}}.
"#.to_string());

    let mut scripts = HashMap::new();
    scripts.insert("build".to_string(), "cargo build".to_string());
    scripts.insert("run".to_string(), "cargo run".to_string());
    scripts.insert("test".to_string(), "cargo test".to_string());
    scripts.insert("check".to_string(), "cargo check".to_string());
    scripts.insert("clippy".to_string(), "cargo clippy".to_string());
    scripts.insert("fmt".to_string(), "cargo fmt".to_string());

    let mut aliases = HashMap::new();
    aliases.insert("b".to_string(), "cargo build".to_string());
    aliases.insert("r".to_string(), "cargo run".to_string());
    aliases.insert("t".to_string(), "cargo test".to_string());
    aliases.insert("c".to_string(), "cargo check".to_string());

    ProjectTemplate {
        name: "rust-cli".to_string(),
        description: "Rust command-line application with Clap".to_string(),
        version: "1.0.0".to_string(),
        author: "Cyrus Templates".to_string(),
        language: "rust".to_string(),
        language_version: "1.75".to_string(),
        package_manager: "cargo".to_string(),
        metadata: TemplateMetadata {
            category: TemplateCategory::Cli,
            tags: vec!["rust".to_string(), "cli".to_string(), "clap".to_string()],
            difficulty: DifficultyLevel::Beginner,
            min_cyrus_version: "0.3.0".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            documentation: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        files,
        dependencies: vec![
            "clap".to_string(),
            "anyhow".to_string(),
            "serde".to_string(),
            "tokio".to_string(),
        ],
        dev_dependencies: vec![
            "assert_cmd".to_string(),
            "predicates".to_string(),
        ],
        scripts,
        aliases,
        environment: HashMap::new(),
        post_install_commands: vec![
            PostInstallCommand {
                command: "cargo".to_string(),
                args: vec!["build".to_string()],
                working_directory: None,
                condition: None,
                ignore_failure: false,
            }
        ],
        variables: HashMap::new(),
        features: vec![],
        hooks: TemplateHooks::default(),
    }
}

pub fn create_python_api_template() -> ProjectTemplate {
    let mut files = HashMap::new();

    files.insert("main.py".to_string(), r#""""
{{project_name}} - FastAPI application built with Cyrus
Author: {{author}}
"""

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Optional
import uvicorn

app = FastAPI(
    title="{{project_name}}",
    description="A FastAPI application built with Cyrus",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc",
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Data models
class Item(BaseModel):
    id: Optional[int] = None
    name: str
    description: Optional[str] = None
    price: float
    in_stock: bool = True

class ItemCreate(BaseModel):
    name: str
    description: Optional[str] = None
    price: float

# In-memory storage (replace with database in production)
items_db: List[Item] = []
next_id = 1

@app.get("/")
async def root():
    return {
        "message": "Welcome to {{project_name}}!",
        "version": "1.0.0",
        "author": "{{author}}",
        "docs": "/docs"
    }

@app.get("/health")
async def health_check():
    return {"status": "healthy", "service": "{{project_name}}"}

@app.get("/items", response_model=List[Item])
async def get_items():
    return items_db

@app.get("/items/{item_id}", response_model=Item)
async def get_item(item_id: int):
    item = next((item for item in items_db if item.id == item_id), None)
    if not item:
        raise HTTPException(status_code=404, detail="Item not found")
    return item

@app.post("/items", response_model=Item)
async def create_item(item: ItemCreate):
    global next_id
    new_item = Item(id=next_id, **item.dict())
    items_db.append(new_item)
    next_id += 1
    return new_item

@app.put("/items/{item_id}", response_model=Item)
async def update_item(item_id: int, item_update: ItemCreate):
    item = next((item for item in items_db if item.id == item_id), None)
    if not item:
        raise HTTPException(status_code=404, detail="Item not found")

    for key, value in item_update.dict().items():
        setattr(item, key, value)

    return item

@app.delete("/items/{item_id}")
async def delete_item(item_id: int):
    global items_db
    items_db = [item for item in items_db if item.id != item_id]
    return {"message": "Item deleted successfully"}

if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )
"#.to_string());

    files.insert("requirements.txt".to_string(), r#"fastapi==0.104.1
uvicorn[standard]==0.24.0
pydantic==2.5.0
python-multipart==0.0.6
"#.to_string());

    files.insert("tests/test_main.py".to_string(), r#"import pytest
from fastapi.testclient import TestClient
from main import app

client = TestClient(app)

def test_root():
    response = client.get("/")
    assert response.status_code == 200
    data = response.json()
    assert "message" in data
    assert "{{project_name}}" in data["message"]

def test_health_check():
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json()["status"] == "healthy"

def test_create_item():
    item_data = {
        "name": "Test Item",
        "description": "A test item",
        "price": 10.99
    }
    response = client.post("/items", json=item_data)
    assert response.status_code == 200
    data = response.json()
    assert data["name"] == item_data["name"]
    assert data["id"] is not None

def test_get_items():
    response = client.get("/items")
    assert response.status_code == 200
    assert isinstance(response.json(), list)
"#.to_string());

    files.insert("README.md".to_string(), r#"# {{project_name}}

A FastAPI application built with Cyrus.

## Features

- 🚀 FastAPI with automatic API documentation
- 🔄 CORS support
- 📝 Pydantic models for request/response validation
- 🧪 Test suite with pytest
- 📚 Interactive API docs at `/docs`

## Getting Started

1. Install dependencies:
   ```bash
   cyrus run install
   ```

2. Run the development server:
   ```bash
   cyrus run dev
   ```

3. Visit the API docs:
   - Swagger UI: http://localhost:8000/docs
   - ReDoc: http://localhost:8000/redoc

## API Endpoints

- `GET /` - Welcome message
- `GET /health` - Health check
- `GET /items` - List all items
- `POST /items` - Create a new item
- `GET /items/{id}` - Get item by ID
- `PUT /items/{id}` - Update item
- `DELETE /items/{id}` - Delete item

## Development

```bash
# Run tests
cyrus run test

# Format code
cyrus run format

# Lint code
cyrus run lint
```

## Author

Created by {{author}} in {{current_year}}.
"#.to_string());

    let mut scripts = HashMap::new();
    scripts.insert("dev".to_string(), "uvicorn main:app --reload --host 0.0.0.0 --port 8000".to_string());
    scripts.insert("start".to_string(), "uvicorn main:app --host 0.0.0.0 --port 8000".to_string());
    scripts.insert("test".to_string(), "pytest".to_string());
    scripts.insert("install".to_string(), "pip install -r requirements.txt".to_string());
    scripts.insert("format".to_string(), "black .".to_string());
    scripts.insert("lint".to_string(), "flake8".to_string());

    ProjectTemplate {
        name: "python-api".to_string(),
        description: "Python FastAPI application with CRUD operations".to_string(),
        version: "1.0.0".to_string(),
        author: "Cyrus Templates".to_string(),
        language: "python".to_string(),
        language_version: "3.11".to_string(),
        package_manager: "pip".to_string(),
        metadata: TemplateMetadata {
            category: TemplateCategory::Api,
            tags: vec!["python".to_string(), "fastapi".to_string(), "api".to_string()],
            difficulty: DifficultyLevel::Intermediate,
            min_cyrus_version: "0.3.0".to_string(),
            license: "MIT".to_string(),
            repository: None,
            homepage: None,
            documentation: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        files,
        dependencies: vec![
            "fastapi".to_string(),
            "uvicorn[standard]".to_string(),
            "pydantic".to_string(),
            "python-multipart".to_string(),
        ],
        dev_dependencies: vec![
            "pytest".to_string(),
            "black".to_string(),
            "flake8".to_string(),
            "httpx".to_string(),
        ],
        scripts,
        aliases: HashMap::new(),
        environment: HashMap::new(),
        post_install_commands: vec![
            PostInstallCommand {
                command: "pip".to_string(),
                args: vec!["install".to_string(), "-r".to_string(), "requirements.txt".to_string()],
                working_directory: None,
                condition: None,
                ignore_failure: false,
            }
        ],
        variables: HashMap::new(),
        features: vec![],
        hooks: TemplateHooks::default(),
    }
}