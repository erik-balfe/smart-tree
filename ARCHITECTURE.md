# Smart Tree Architecture

This document describes the high-level architecture of the Smart Tree application, focusing on the core components and their interactions.

## Component Overview

Smart Tree is organized into several key modules:

```
smart-tree/
├── src/
│   ├── main.rs           # Entry point and CLI parsing
│   ├── lib.rs            # Public API and re-exports
│   ├── scanner.rs        # Directory traversal and metadata collection
│   ├── gitignore.rs      # Gitignore pattern handling
│   ├── rules.rs          # Smart filtering rules system
│   ├── types.rs          # Core data structures
│   ├── display/          # Display formatting and visualization
│   │   ├── mod.rs
│   │   ├── colors.rs
│   │   ├── state.rs      # Display state management
│   │   ├── utils.rs
│   │   └── tests.rs
│   └── tests/            # Integration tests
└── test_data/            # Sample data for examples and tests
```

## Core Components

### 1. Directory Scanning (scanner.rs)

The scanning subsystem is responsible for traversing the directory structure, collecting metadata, and building the tree representation. Key responsibilities:

- Efficient directory traversal with depth limiting
- Metadata collection (size, timestamps, file counts)
- Applying .gitignore rules during traversal
- Handling errors for inaccessible files/directories

### 2. Gitignore Implementation (gitignore.rs)

The gitignore subsystem handles pattern matching for ignored files and directories:

- Parse .gitignore patterns from files
- Apply pattern matching rules
- Support for nested .gitignore files
- Handle system directories and special patterns

### 3. Smart Filtering Rules (rules.rs)

The rules engine provides context-aware filtering based on project types and content:

- Project type detection (Rust, Node.js, etc.)
- Pluggable rule system for extensibility
- Contextual evaluation of directories and files
- Weighted scoring for complex decisions

### 4. Display System (display/)

The display subsystem handles the visual formatting of the tree structure:

- Intelligent line allocation with budget management
- Head/tail display patterns for large listings
- Color and emoji support for visual enhancement
- Formatting of metadata and tree structure

## Data Flow

1. **Input Processing**:
   - Command-line arguments are parsed in `main.rs`
   - Configuration objects are created

2. **Directory Scanning**:
   - `scanner.rs` traverses the specified directory
   - Applies gitignore rules during traversal
   - Builds a `DirectoryEntry` tree structure

3. **Smart Filtering**:
   - Rules engine analyzes paths and context
   - Determines what to show and what to fold
   - Applies project-specific intelligence

4. **Tree Rendering**:
   - Display state allocates line budget
   - Rendering logic applies folding decisions
   - Formatting with colors and metadata
   - Output to terminal

## Extensibility Points

Smart Tree is designed with several extensibility mechanisms:

1. **Filtering Rules**:
   - Custom rules can be added to the `FilterRegistry`
   - Rules implement the `FilterRule` trait
   - Project-specific filtering based on detected context

2. **Display Formats**:
   - The formatting system is modular
   - Support for different output types (terminal, JSON, HTML)
   - Customizable metadata formatting

3. **Configuration**:
   - User-defined configuration via `.smart-tree.toml`
   - Runtime configuration through CLI options
   - Project-specific configurations

## Design Principles

1. **Intelligent Display**:
   The core value proposition is making smart decisions about what to show, not just showing everything.

2. **Context Awareness**:
   Understanding project types and patterns to make better filtering decisions.

3. **Progressive Disclosure**:
   Show important information first, with options to reveal more detail.

4. **Visual Clarity**:
   Use color, spacing, and symbols to enhance readability and information density.

5. **Performance**:
   Fast scanning even for large directories, with optimizations for common cases.