# Cyrus - Herramienta de Gestión de Lenguajes Todo-en-Uno

Cyrus es una herramienta integral para gestionar entornos de lenguajes de programación con aislamiento de proyectos locales y capacidades de instalación global.

## Características

- 🚀 **Soporte multi-lenguaje**: Python, JavaScript/Node.js, Go
- 🔧 **Aislamiento de proyectos locales**: Cada proyecto tiene su propio entorno
- 📦 **Integración de gestores de paquetes**: Soporte para pip, npm, yarn, poetry, etc.
- 🌐 **Gestión global de lenguajes**: Instalar lenguajes en ~/.cyrus
- ⚡ **Rápido y liviano**: Construido con Rust para máximo rendimiento
- 🛠️ **Arquitectura modular**: Fácil de extender con nuevos lenguajes

## Instalación

Descarga la última versión para tu plataforma o compila desde el código fuente:

```bash
cargo build --release
```

## Inicio Rápido

### Instalación global de lenguajes
```bash
cyrus install python3.11
cyrus install node20
cyrus install go1.21
```

### Inicializar nuevo proyecto
```bash
mkdir my-project && cd my-project
cyrus init
```

### Ejecutar comandos en el entorno del proyecto
```bash
cyrus run python app.py
cyrus run npm start
cyrus run go build
```

## Autor

**Omid Nateghi** - Creador y mantenedor
**Motor**: Omid Coder
