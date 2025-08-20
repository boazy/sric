# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SRIC is a Rust CLI tool that automatically generates Subresource Integrity (SRI) hashes for HTML files. It parses HTML documents, identifies external resources (scripts and stylesheets), fetches them over HTTP, and adds integrity attributes with SHA-384 hashes.

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run -- <html-file>` - Run on an HTML file and output to stdout
- `cargo run -- <html-file> -w` - Modify the HTML file in-place with SRI hashes
- `cargo run -- <html-file> -o <output-file>` - Write output to specific file
- `cargo run -- <html-file> --force` - Override existing SRI hashes

### Testing and Quality
- `cargo test` - Run unit tests
- `cargo check` - Check for compilation errors without building
- `cargo clippy` - Run linter (if installed)
- `cargo fmt` - Format code (if installed)

## Architecture

### Core Modules
- `main.rs` - CLI argument parsing using clap and clio, orchestrates the SRI generation process
- `parse_html.rs` - HTML parsing using html5ever, creates DOM from input files
- `generate_sri.rs` - Core SRI generation logic, fetches remote resources and computes SHA-384 hashes
- `write_html.rs` - Serializes modified DOM back to HTML
- `node_iter.rs` - DOM traversal utilities and attribute manipulation helpers
- `element.rs` - Element-specific operations and utilities
- `mime_ext.rs` - MIME type validation extensions
- `response_ext.rs` - HTTP response processing extensions

### Key Dependencies
- `html5ever` + `markup5ever_rcdom` - HTML parsing and DOM manipulation
- `ureq` - HTTP client for fetching external resources
- `sha2` + `base64` - Cryptographic hashing for SRI generation
- `clap` + `clio` - CLI interface and file I/O handling
- `mime` + `mime_guess` - Content type validation

### SRI Generation Process
1. Parse HTML into DOM using html5ever
2. Traverse nodes to find `<script>` tags with `src` and `<link>` tags with `rel="stylesheet|preload|modulepreload"`
3. For each external resource, fetch content via HTTP and validate MIME type
4. Generate SHA-384 hash and encode as base64
5. Add or update `integrity` attribute with `sha384-<hash>` format
6. Serialize modified DOM back to HTML

### Security Features
- MIME type validation prevents SRI generation for unexpected content types
- 10MB download limit for safety
- HTTPS upgrade for HTTP URLs
- Content-Type header verification against file extension