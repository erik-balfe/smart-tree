# Smart Tree Viewer

A modern directory tree viewer that prioritizes readability by intelligently displaying content. Unlike traditional `tree` command, it makes smart decisions about what to show and what to fold, ensuring the output remains useful even for large directory structures.

## Why Another Tree Viewer?

Standard `tree` command has served well for decades, but it has limitations when working with modern project structures:

- Outputs everything, often producing thousands of lines for a simple project
- Important recent changes get lost in the noise of build artifacts and dependencies
- No smart prioritization - latest changes and important files get equal treatment with old cached files
- Output often gets truncated by terminal or becomes too large to scan effectively

Consider a typical Node.js project:

```
$ tree
.
├── node_modules/ (1,500+ files)
├── src/
├── dist/
├── .git/ (also many senseless files)
└── ...
```

By the time you see your source files, you've scrolled past hundreds of dependency files.

## How Smart Tree Helps

Smart Tree takes a different approach:

1. **Intelligent Folding**

   - Automatically folds known system directories (.git, node_modules)
   - Shows summary for large directories instead of cluttering output
   - Preserves context by showing directory sizes and file counts

2. **Line Limit with Context**

   - Respects terminal limitations while maintaining usefulness
   - When sorting by date, shows both newest AND oldest files
   - Ensures you see the most relevant files within the limit

3. **Rich Metadata and Visualization**

   - File sizes in human-readable format
   - Modification times in relative format ("5m ago", "2h ago")
   - Directory statistics (file counts, total sizes)
   - Color-coded file types (directories, code, images, etc.)
   - Emoji icons for different file types
   - Size-based color gradients for quick visual assessment
   - Detailed metadata view with file type information

## Usage Examples

Smart Tree helps in different scenarios where traditional tree command becomes impractical. Here are some common use cases:

### Enhanced Visual Display

Smart Tree offers rich visualization options to make browsing directories more intuitive:

```bash
# Use emoji icons and colored metadata
$ smart-tree --emoji --color-sizes --color-dates

# Detailed view with file type information and enhanced formatting
$ smart-tree --detailed --emoji

# Choose between light and dark theme
$ smart-tree --color-theme dark --emoji
```

### Finding Recent Changes

When you need to see what's changed recently in a project:

```bash
# Traditional tree - overwhelming output
$ tree
.
├── node_modules (1500+ files...)
├── src
│   ├── components (200+ files...)
│   └── ... hundreds more lines
└── ... output truncated

# Smart Tree - immediately shows what matters
$ smart-tree --sort-by modified --max-lines 10
.
├── src/ (215 files, 1.2MB, modified 5m ago)
│   ├── components/Button.js (modified 2m ago)
│   ├── components/Form.js (modified 5m ago)
│   └── ... 3 more recent files
├── package.json (modified 1h ago)
├── node_modules/ (1.5k files, 250MB) [folded: system]
└── ... 3 more items
```

### Large Project Navigation

For large projects, Smart Tree helps maintain context:

```bash
# Default view (shows overview)
$ smart-tree --max-lines 15
.
├── src/ (120 files, 5.2MB)
│   ├── main.rs (2.1MB)
│   ├── lib.rs (1.5MB)
│   └── ... 5 more items
├── tests/ (25 files, 500KB)
├── target/ (2.3k files, 1.2GB) [folded: system]
└── ... 2 more items

# Detailed view of specific directory
$ smart-tree src --sort-by size --max-lines 10
src
├── main.rs (2.1MB, modified 1h ago)
├── lib.rs (1.5MB, modified 2h ago)
├── modules/ (15 files, 800KB)
│   ├── largest.rs (300KB)
│   └── ... 3 more items
└── ... 2 more items
```

### Finding Large Files

When you need to clean up disk space:

```bash
$ smart-tree --sort-by size --max-lines 12
.
├── target/ (2.3GB) [folded: system]
├── videos/ (1.5GB)
│   ├── demo.mp4 (1.2GB)
│   └── intro.mp4 (300MB)
├── src/ (5.2MB)
│   ├── assets/image.png (3MB)
│   └── ... 4 more items
└── ... 3 more items
```

### Line Limit Examples

Smart Tree adapts its output based on available space while maintaining usefulness:

```bash
# Very limited space - shows critical overview
$ smart-tree --max-lines 5
.
├── src/ (120 files, 5.2MB, modified 5m ago)
├── target/ (2.3k files, 1.2GB) [folded: system]
└── ... 3 more items

# More space - adds detail while staying readable
$ smart-tree --max-lines 15
.
├── src/ (120 files, 5.2MB)
│   ├── main.rs (modified 2m ago)
│   ├── lib.rs (modified 5m ago)
│   └── ... 3 more items
├── tests/ (25 files)
│   ├── integration.rs (modified 1h ago)
│   └── ... 2 more items
├── target/ (2.3k files, 1.2GB) [folded: system]
├── Cargo.toml (modified 1d ago)
└── ... 2 more items
```

## Key Features

- **Smart Directory Scanning**: Efficient recursive directory traversal with gitignore support
- **Configurable Output**: Control display using max-lines, dir-limit, and sorting options
- **Intelligent Folding**: Automatically collapses system directories like .git and node_modules
- **Rich Metadata Display**: Shows file sizes, modification times, and directory statistics
- **Context-Aware Output**: Maintains context by showing "X more items" summary for large folders 
- **Multiple Sort Options**: Sort by name, size, or modification time with directory grouping

## Design Philosophy

Smart Tree is built around several key principles:

1. **Information Density Over Completeness**

   - Showing every file in large directories rarely helps and often obscures important information
   - Smart Tree prioritizes the most relevant files (newest, largest) while summarizing the rest
   - System directories (.git, target, node_modules) are automatically folded to reduce noise

2. **Context Preservation**

   - Even with strict line limits, Smart Tree maintains directory structure context
   - When showing only a subset of files, summaries indicate what's not shown
   - Important metadata (sizes, counts, timestamps) help maintain context even in compact views

3. **Smart Defaults with Configurability**
   - Sane defaults make the tool immediately useful without configuration
   - Common system directories are auto-detected and folded
   - All aspects can be customized through command-line options (sorting, line limits, directory limits)

## Technical Details

### Architecture

1. **Core Components**

   - Scanner: Directory traversal and metadata collection
   - Display Engine: Output formatting and line management
   - Folding Logic: Decision making for content display

2. **Key Algorithms**

   - Directory traversal strategy
   - Line limit management
   - Sort and display priority calculation

3. **Data Structures**
   - Directory tree representation
   - Metadata caching
   - Display state management

### Implementation Details

1. **Folding Strategy**

   - System directory detection
   - Size-based folding rules
   - Dynamic adjustment based on available space

2. **Line Management**

   - How line limits are enforced
   - Priority calculation for limited space
   - Balance between levels of directory tree

3. **Performance Considerations**
   - Lazy loading for large directories
   - Metadata caching
   - Memory usage optimization

### Development Setup

To set up the project for development:

```bash
# Clone the repository
git clone https://github.com/erik-balfe/smart-tree.git
cd smart-tree

# Build the project
cargo build

# Run tests
cargo test

# Run the binary
cargo run -- [path] [options]
```

#### Testing Approach

The project uses Rust's built-in testing framework with:

- Unit tests alongside implementation code
- Integration tests for end-to-end functionality
- Pretty assertions for more readable test output

### Future Improvements

Planned enhancements for future versions:

- **Terminal Width Consideration**: Adaptive formatting based on available terminal width
- **Color Support**: Syntax highlighting for different file types and attributes
- **Parallel Scanning**: Multi-threaded directory traversal for large repositories
- **Metadata Caching**: Store and reuse file metadata for frequently accessed directories
- **Custom Folding Rules**: User-configurable rules for auto-folding directories
- **Interactive Mode**: Navigate the tree interactively with keyboard controls

## Installation

### Quick Install (Linux, macOS)

```bash
# Easy install script
curl -sSL https://raw.githubusercontent.com/erik-balfe/smart-tree/master/install.sh | sh
```

### macOS via Homebrew

```bash
# Homebrew installation will be available soon
brew install erik-balfe/tap/smart-tree
```

### Manual Installation

```bash
# Install from source
git clone https://github.com/erik-balfe/smart-tree.git
cd smart-tree
cargo install --path .

# Now you can run from anywhere
smart-tree [path] [options]
```

### Pre-built Binaries

You can also download pre-built binaries for your platform from the [GitHub Releases page](https://github.com/erik-balfe/smart-tree/releases).

## Contributing

We welcome contributions to Smart Tree! The project has a full CI/CD pipeline set up with GitHub Actions that:

- Runs tests automatically on every push and pull request
- Checks code formatting with `rustfmt`
- Runs the `clippy` linter for code quality
- Generates binaries for multiple platforms when a release is triggered

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed instructions on:
- Setting up your development environment
- Our branching strategy and workflow
- Code style guidelines
- Testing requirements
- How to create releases

## Usage

```bash
# Basic usage - current directory
smart-tree

# Specify directory
smart-tree /path/to/directory

# Control output size
smart-tree --max-lines 50

# Sort by modification time
smart-tree --sort-by modified

# Sort by size
smart-tree --sort-by size

# Limit directory depth
smart-tree -L 3

# Show help with all options
smart-tree --help
```

## Project Structure

```
src/
├── display/            # Output formatting and display logic
│   ├── mod.rs          # Module interface
│   ├── format.rs       # Tree formatting implementation
│   ├── state.rs        # Display state management
│   ├── tests.rs        # Display tests
│   └── utils.rs        # Formatting utilities
├── gitignore.rs        # Gitignore pattern handling
├── lib.rs              # Library interface
├── log_macros.rs       # Logging utilities
├── main.rs             # CLI interface
├── scanner.rs          # Directory traversal
└── types.rs            # Core data structures
```

### Core Components

1. **Scanner (`scanner.rs`)**

   - Traverses directory structure recursively
   - Collects metadata (size, modification times)
   - Builds `DirectoryEntry` tree structure
   - Handles directory depth limits
   - Integrates with gitignore patterns

2. **Display Engine (`display/`)**

   - **State Management (`state.rs`)**

     - Tracks remaining lines
     - Manages output buffer
     - Implements smart folding decisions

   - **Formatting (`format.rs`)**

     - Implements tree-like structure formatting
     - Handles indentation and connection lines
     - Manages entry presentation

   - **Utilities (`utils.rs`)**
     - Human-readable size formatting
     - Time formatting
     - Entry sorting implementation

3. **Data Structures (`types.rs`)**
   ```rust
   DirectoryEntry {
       path: PathBuf,
       name: String,
       is_dir: bool,
       metadata: EntryMetadata,
       children: Vec<DirectoryEntry>,
       is_gitignored: bool,
   }
   ```

- Represents directory tree structure
- Stores file/directory metadata
- Supports recursive tree building

### Key Algorithms

1.  **Directory Traversal**

    - Breadth-first scanning for better memory management
    - Metadata collection during traversal
    - Gitignore pattern matching
    - Size accumulation for directories

2.  **Display Strategy**
    - Line budget management
    - Priority-based content selection
    - Dynamic folding based on available space
    - Context preservation in sorted output


### Current Implementation Approaches

1.  **Line Management**

    - Pre-calculation of required lines
    - Dynamic adjustment based on content
    - Preservation of important content
    - Smart folding of large directories

2.  **Sorting Implementation**

    - Configurable sort criteria (time, size, name)
    - Directory grouping option
    - Preservation of both ends in sorted lists

3.  **State Management**
    - Single-pass display generation
    - Stateful output building
    - Line budget tracking

### Current Limitations and Areas for Improvement

1.  **Performance**

    - Large directory scanning could be optimized
    - Metadata caching not implemented yet
    - Potential for parallel scanning

2.  **Display Engine**

    - Limited handling of very deep structures
    - No terminal width consideration yet
    - Color support not implemented

3.  **Memory Usage**
    - Full directory tree kept in memory
    - Could benefit from streaming approach for very large directories

