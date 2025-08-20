# SRIC - Subresource Integrity Generator

A Rust CLI tool that automatically generates Subresource Integrity (SRI) hashes for HTML files. SRIC parses HTML documents, identifies external resources (scripts and stylesheets), fetches them over HTTP, and adds integrity attributes with SHA-384 hashes.

## Features

- Generates SHA-384 SRI hashes for external scripts and stylesheets
- Supports in-place modification or output to new files
- MIME type validation for security
- HTTPS upgrade for HTTP URLs
- Content-Type header verification
- 10MB download limit for safety
- Force override of existing SRI hashes

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Output to stdout
cargo run -- <html-file>

# Modify file in-place
cargo run -- <html-file> -w

# Write to specific output file
cargo run -- <html-file> -o <output-file>

# Override existing SRI hashes
cargo run -- <html-file> --force
```

## Examples

```bash
# Generate SRI hashes and print to stdout
cargo run -- index.html

# Update index.html in-place with SRI hashes
cargo run -- index.html -w

# Save output to a new file
cargo run -- index.html -o index-with-sri.html
```

## Development

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check for compilation errors
cargo check

# Run linter (if installed)
cargo clippy

# Format code (if installed)
cargo fmt
```

### Architecture

- `main.rs` - CLI argument parsing and orchestration
- `parse_html.rs` - HTML parsing using html5ever
- `generate_sri.rs` - Core SRI generation logic
- `write_html.rs` - HTML serialization
- `node_iter.rs` - DOM traversal utilities
- `element.rs` - Element operations
- `mime_ext.rs` - MIME type validation
- `response_ext.rs` - HTTP response processing

### Dependencies

- `html5ever` + `markup5ever_rcdom` - HTML parsing and DOM manipulation
- `ureq` - HTTP client for fetching resources
- `sha2` + `base64` - Cryptographic hashing
- `clap` + `clio` - CLI interface and file I/O
- `mime` + `mime_guess` - Content type validation

## How It Works

1. Parse HTML into DOM using html5ever
2. Find `<script>` tags with `src` and `<link>` tags with `rel="stylesheet|preload|modulepreload"`
3. Fetch external resources via HTTP with MIME type validation
4. Generate SHA-384 hash and encode as base64
5. Add or update `integrity` attribute with `sha384-<hash>` format
6. Serialize modified DOM back to HTML

## Security

- MIME type validation prevents SRI generation for unexpected content types
- 10MB download limit for safety
- HTTPS upgrade for HTTP URLs
- Content-Type header verification against file extension