# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build/Lint/Test Commands

### Building
- Build debug version: `cargo build`
- Build release version: `cargo build --release`
- Run the application: `cargo run`

### Testing
- Run all tests: `cargo test`
- Run a specific test: `cargo test test_name`
- Run tests for a specific module: `cargo test module_name`
- Run tests with feature flags: `cargo test --features test_utils`

### Linting
- Run clippy linting: `cargo clippy`
- Format code: `cargo fmt`

## Code Style Guidelines

### Architecture
- This project follows a hexagonal/clean architecture pattern:
  - `application`: Contains services (use cases), DTOs, and ports
  - `domain`: Contains core entities, repositories (interfaces), and domain services
  - `infrastructure`: Contains concrete implementations of repository interfaces
  - `interfaces`: Contains HTTP/API handlers and routes

### Error Handling
- Use the `DomainError` type for domain-level errors
- Use the `AppError` type for API/HTTP-level errors
- Use the `ErrorContext` trait to add context to errors from external crates
- Follow the error factory pattern for creating common error types

### Naming Conventions
- Types and structs: PascalCase
- Functions and methods: snake_case
- Constants and statics: SCREAMING_SNAKE_CASE
- Modules and files: snake_case
- Use descriptive names that express intent

### Testing
- Use mock objects for dependencies in unit tests
- Use the `#[tokio::test]` attribute for async tests
- Include both positive and negative test cases
- Follow the Arrange-Act-Assert pattern in tests

### Imports
- Group imports by source:
  1. Standard library imports
  2. External crate imports
  3. Local crate imports (with `crate::` prefix)
- Use explicit imports (no glob imports except in tests)

### Documentation
- Document public API functions and types with doc comments
- Include examples where helpful
- Document error cases and conditions