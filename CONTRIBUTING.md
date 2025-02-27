# Contributing to Smart Tree

Thank you for your interest in contributing to Smart Tree! This document provides guidelines and instructions for contributing to the project.

## Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/erik-balfe/smart-tree.git
   cd smart-tree
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Run the formatter and linter:**
   ```bash
   cargo fmt
   cargo clippy
   ```

## Development Workflow

### Branching Strategy

- `master` branch is the main development branch
- For releases, we create version branches like `v0.1.0`
- For features or bug fixes, create feature branches from `master`

### Making Changes

1. Create a new branch for your changes
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit them with a descriptive message
   ```bash
   git commit -m "Meaningful commit message describing the change"
   ```

3. Push your branch to GitHub
   ```bash
   git push -u origin feature/your-feature-name
   ```

4. Create a pull request targeting the `master` branch

### Continuous Integration

The project uses GitHub Actions for continuous integration with the following checks:

- **Test:** Runs the test suite
- **Rustfmt:** Checks code formatting
- **Clippy:** Performs static analysis

All CI checks must pass before a pull request can be merged.

### Release Process

Releases are automated through GitHub Actions:

1. Create and push a tag for the version (e.g., `v0.1.0`)
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```
2. The workflow will be triggered automatically and:
   - Run tests, formatting checks, and linter
   - Build binaries for Linux, macOS, and Windows
   - Create a GitHub Release with the binaries attached

## Code Guidelines

### Formatting and Style

- Follow Rust standard formatting with `cargo fmt`
- Address all clippy warnings with `cargo clippy`
- Organize imports: standard lib first, then external crates, then internal modules
- Use meaningful type names in CamelCase, functions/variables in snake_case

### Documentation

- Document public functions with doc comments (//!)
- Keep the README up to date
- Update DEVELOPMENT_PLAN.md when adding new features

### Testing

- Write tests for all new functionality
- Maintain the existing test structure with test modules co-located with implementation
- Use descriptive test names and assertions

## Issue Reporting

When reporting issues, please include:

1. Steps to reproduce the issue
2. Expected behavior
3. Actual behavior
4. Version information (OS, Rust version)

## Feature Requests

Feature requests are welcome! When suggesting a feature, please:

1. Explain the problem you're facing
2. Describe the solution you'd like
3. Discuss alternatives you've considered

## License

By contributing to this project, you agree that your contributions will be licensed under the project's MIT license.