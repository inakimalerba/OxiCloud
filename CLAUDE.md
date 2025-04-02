# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- `cargo build` - Build the project
- `cargo run` - Run OxiCloud locally
- `cargo build --release` - Build with full optimization
- `cargo run --release` - Run optimized build
- `RUST_LOG=debug cargo run` - Run with detailed logging

## Test Commands
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test trash_service_test::tests::test_move_file_to_trash`)
- `cargo bench` - Run benchmarks with optimized settings

## Lint Commands
- `cargo clippy` - Run linter with Rust's default Clippy lints
- `cargo fmt` - Format code according to Rust style guidelines

## Code Style Guidelines
- Use async/await for asynchronous operations with tokio runtime
- Follow Rust naming conventions: snake_case for variables/functions, CamelCase for types
- Use thiserror for error handling with comprehensive error types
- Document all public functions with doc comments (/** ... */)
- Use the #[async_trait] attribute for async trait implementations
- Follow clean architecture principles with clear layer separation (domain, application, infrastructure, interfaces)
- Use Result types with specific error enums for error handling
- Keep functions small and focused on a single responsibility
- Prefer immutable variables (let) over mutable ones (let mut) when possible