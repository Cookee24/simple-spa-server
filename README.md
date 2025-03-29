# Simple SPA Server

A lightweight HTTP server designed to serve Single Page Applications (SPAs) with minimal configuration. It can either serve files from a directory or bundle them directly into the binary.

## Features

- Serve static files from the filesystem
- Option to bundle assets into the binary (no need for separate files)
- SPA mode that falls back to index.html for client-side routing
- Configurable listening address
- Request logging
- Small memory footprint

## Installation

### From Source

1. Clone the repository:

   ```
   git clone <repository-url>
   cd simple-spa-server
   ```

2. Build with Cargo:

   ```
   # Standard build (serves from filesystem)
   cargo build --release

   # Build with bundled assets
   cargo build --release --features bundle

   # Build with bundled assets and compression
   cargo build --release --features bundle,compression
   ```

## Usage

### Basic Usage

```
simple-spa-server [OPTIONS]
```

### Command Line Options

- `--listen`, `-l`: The listen address for the server (default: ":8080")
- `--serve-dir`, `-s`: The directory to serve files from (default: ".")
- `--index`: Whether to return index.html for unmatched routes (default: true)
- `--blocking-threads`: Maximum number of blocking threads (default: 8)

### Examples

```bash
# Serve the current directory on port 8080
simple-spa-server

# Serve a specific directory on a different port
simple-spa-server --serve-dir ./dist --listen :3000

# Disable SPA mode (don't fall back to index.html)
simple-spa-server --no-index
```

## Bundling Assets

When compiled with the `bundle` feature, your static files must be placed in a `www/` directory at the project root before building. These files will be embedded directly into the binary.

```bash
# Create www directory and add your files
mkdir -p www
cp -r your-spa-build/* www/

# Build with bundling
cargo build --release --features bundle
```
