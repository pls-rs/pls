# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`pls` is a prettier and powerful `ls(1)` replacement written in Rust. It's designed to be a modern, fast, and feature-rich directory listing tool with extensive customization options.

## Architecture

The codebase is organized into several key modules:

- **Core**: `src/main.rs` contains the application entry point with a global `PLS` instance
- **Models**: `src/models/` contains core data structures including `Pls` (main application state), `Node` (file system entries), and `Window` (terminal info)
- **Configuration**: `src/config/` handles YAML configuration files and command-line argument parsing
- **Arguments**: `src/args/` manages command-line input processing and grouping
- **Output**: `src/output/` handles different display formats (grid, table, etc.)
- **Formatting**: `src/fmt/` provides text rendering and markup processing
- **Graphics**: `src/gfx/` handles terminal graphics protocol support (Kitty, SVG)
- **Enums**: `src/enums/` contains various enumeration types for appearance, sorting, etc.
- **Traits**: `src/traits/` defines common behaviors across different components
- **Utils**: `src/utils/` provides utility functions for paths, URLs, and vectors

## Development Commands

### Rust (Core Application)
- **Build**: `cargo build`
- **Run**: `cargo run -- [args]` or `just run [args]`
- **Test**: `cargo test` or `just test`
- **Debug**: `RUST_LOG=debug cargo run -- [args]` or `just debug [args]`
- **Release**: `cargo build --release` or `just release`

### Frontend/Documentation (JavaScript/TypeScript)
- **Lint**: `pnpm lint` (ESLint)
- **Format**: `pnpm format` (Prettier)
- **Check formatting**: `pnpm format:check`
- **All checks**: `pnpm checks` (lint + format check)
- **Fix linting**: `pnpm lint:fix`

### Project Management
- **Install dependencies**: `just install` (installs for all sub-projects)
- **Pre-commit setup**: `just pre-commit` (installs Git hooks)
- **Lint all**: `just lint` (runs pre-commit on all files)

## Key Configuration

The application uses:
- **Configuration files**: `.pls.yml` files for customization (handled by `ConfMan`)
- **Command-line args**: Parsed via `clap` crate
- **Environment**: `RUST_LOG` for debug logging
- **Package manager**: `pnpm` for JavaScript dependencies

## Multi-Language Setup

This is a polyglot project with:
- **Rust**: Main application (`src/`, `Cargo.toml`)
- **JavaScript/TypeScript**: Documentation site (`docs/`, Astro-based)
- **Python**: Examples and utilities (`examples/`, PDM-managed)
- **Just**: Task runner (`justfile` for automation)

## Terminal Graphics

The application detects and supports Kitty's terminal graphics protocol for enhanced visual output. This is handled in `src/gfx/` with runtime detection.