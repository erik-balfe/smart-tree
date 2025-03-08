# Smart Tree Development Plan

## Current Project Status

Smart Tree is a modern directory tree viewer that intelligently displays file hierarchies with an emphasis on readability. Unlike traditional `tree` commands that output everything (often producing thousands of lines), Smart Tree makes smart decisions about what to show and what to fold.

### Implemented Features

- **Core Scanning Engine**: Traverses directories and collects metadata
- **Intelligent Folding**: Automatically folds known system directories (.git, node_modules)
- **Line Budget Management**: Respects terminal limitations while preserving context
- **Head/Tail Display Pattern**: Shows both beginning and end of large listings
- **Sorting Options**: By name, size, modified time, creation time
- **GitIgnore Integration**: Basic folding based on gitignore patterns
- **Metadata Display**: File sizes, modification times, directory statistics
- **CLI Interface**: Configurable options via command line arguments
- **Color and Emoji Support**: Rich visual enhancements for improved readability

### Known Issues

- ✅ ~~Gitignore pattern matching could be further improved~~
- ✅ ~~Performance concerns for large directory structures~~
- ✅ ~~Full directory tree kept in memory (could be problematic for large trees)~~
- ✅ ~~Limited terminal width consideration~~
- ✅ ~~T-shape vs L-shape connectors display incorrectly for the last items~~
- ✅ ~~Program collapses lines where only 1 item is hidden (inefficient)~~
- Contents of .git (and other system directories) cannot be viewed even when explicitly requested
- Does not properly respect nested .gitignore files in subdirectories
- No option to explicitly show system directories that are normally hidden

## Development Roadmap

### Critical Fixes (Immediate Priority)

1. **Fix System Directory Exclusion Logic**
   - Add a flag to explicitly show contents of system directories
   - Modify `display/state.rs` to respect this flag when rendering directories
   - Update state logic to process gitignored directories when explicitly requested

2. **Improve .gitignore Detection and Handling**
   - Implement recursive .gitignore detection to respect nested .gitignore files
   - Load and apply .gitignore rules at each directory level
   - Cache gitignore patterns to avoid redundant file reads
   - Add option to toggle .gitignore consideration

3. **Explicit Directory Inclusion Options**
   - Add command-line parameter to explicitly include directories that would otherwise be folded
   - Support glob patterns for directory inclusion/exclusion
   - Fix logic handling of included directories to properly scan and display their contents

### Phase 1: Code Cleanup and Bug Fixes (Completed) ✅

1. **Fix Test Suite Errors** ✅
   - Remove duplicated test_utils module and ExpectedContent struct
   - Fix duplicated test functions
   - Ensure tests pass with `cargo test`

2. **Fix Code Quality Issues** ✅
   - Address all compiler warnings
   - Apply consistent formatting with `cargo fmt`
   - Run clippy and fix linting issues

3. **Improve Error Handling** ✅
   - Review use of unwrap/expect and replace with proper error handling
   - Improve error messages and recovery

### Phase 2: Core Functionality Enhancement (Current)

1. **Improved GitIgnore Integration** 🚧
   - ✅ Implement proper gitignore pattern matching
   - ✅ Support standard gitignore syntax with globs
   - Add proper recursive .gitignore file detection and pattern application
   - Add option to toggle .gitignore consideration

2. **Memory Optimization** ✅
   - Implement streaming/lazy loading approach for large directories
   - Reduce memory usage for very large directory structures

3. **Performance Improvements** 🚧
   - Optimize directory traversal for large directory structures
   - Implement metadata caching for frequently accessed directories
   - Consider parallel scanning for improved performance

### Phase 3: UI and UX Enhancements (In Progress)

1. **Terminal Integration**
   - Add terminal width consideration for improved display
   - ✅ Implement color support for different file types and metadata
     - ✅ Use different colors for directories, files, symlinks
     - ✅ Highlight gitignored entries in a distinct color
     - ✅ Color code metadata based on file type and size
     - ✅ Add config option to enable/disable colors
     - ✅ Support for light/dark themes
     - ✅ Fix tree connector characters for consistent rendering
   - Add interactive navigation features

2. **Advanced Display Options**
   - ✅ Implement customizable display templates
   - ✅ Add file type recognition and specialized displays
   - ✅ Support different view modes (compact, detailed)
     - ✅ Add detailed metadata display option
     - ✅ Add size-based color gradients
     - ✅ Add date-based color gradients

3. **Visual Enhancements**
   - ✅ Implement icons for file types (emoji support)
   - Add progress indicators for scanning large directories
   - ✅ Improve the visual hierarchy indicators
   
4. **Usability Improvements**
   - Implement shorter command-line aliases (e.g., -n for --max-lines)
   - Evaluate program name and consider alternatives (e.g., "st" for Smart Tree)
   - Improve help messages with clearer examples and option descriptions
   - Add bash/zsh completions for improved user experience

### Phase 4: Distribution and Integration (Medium Term)

1. **Package and Distribution**
   - Create installation script (install.sh) for simple cross-platform installation
   - Package for Homebrew (brew) installation on macOS
   - Implement continuous integration/deployment pipeline
   - Set up automated GitHub releases with versioned branches (e.g., 'v0.2.0')

2. **Integration Features**
   - VCS integration (show modified files since last commit)
   - Build system integration for project-specific views
   - Configuration file support for project defaults
   - Add support for config file in multiple formats (.json, .toml)

### Phase 5: Feature Expansion (Long Term)

1. **Advanced Filtering**
   - Implement complex file filtering based on patterns
   - Add time-based filtering (recently modified, older than X)
   - Support regex-based content filtering

2. **Export Capabilities**
   - JSON/XML export of directory structure
   - HTML visualization export
   - Integration with other tools via standardized output formats

3. **Advanced Integration**
   - Add plugin system for custom extensions
   - Implement extensions for popular development tools
   - Add language-specific statistics gathering

## Implementation Details for Upcoming Work

### Gitignore and System Directory Handling Design

1. **Recursive GitIgnore Detection**
   ```
   App
   ├── .gitignore (ignores *.log)
   ├── src/
   │   ├── .gitignore (ignores *.tmp)
   │   └── components/
   │       ├── .gitignore (ignores *.bak)
   │       └── ...
   └── ...
   ```

   - Each directory's .gitignore rules should apply to itself and all subdirectories
   - Later .gitignore files take precedence over earlier ones
   - Parent rules can be negated by child rules (using ! prefix)

2. **Scanner Implementation**
   ```rust
   pub struct GitIgnoreContext {
       patterns: Vec<GitIgnore>,
   }

   impl GitIgnoreContext {
       // Load initial .gitignore from root
       pub fn new(root: &Path) -> Self { ... }

       // Check if path matches any pattern in any loaded .gitignore
       pub fn is_ignored(&self, path: &Path) -> bool { ... }

       // Process a directory, loading any .gitignore found there
       pub fn process_directory(&mut self, dir_path: &Path) { ... }
   }
   ```

3. **System Directory Handling**
   - Add new CLI option: `--show-system-dirs` to show system directories
   - Modify scanner to mark system dirs but traverse them if explicitly requested
   - Update display logic to respect this setting

## Test Strategy

1. **Unit Tests for GitIgnore Context**
   - Test recursive .gitignore file loading
   - Test pattern application from multiple .gitignore files
   - Test precedence rules between parent and child patterns

2. **Integration Tests with Nested Repositories**
   - Set up test directories with nested git repositories
   - Verify correct traversal behavior with and without explicit inclusion
   - Test with `--show-system-dirs` flag

3. **End-to-End Testing**
   - Create a test suite that verifies real-world nested repository scenarios
   - Test performance with large repositories containing multiple .gitignore files

## Implementation Notes

- Maintain modular architecture with clear separation of concerns
- Write tests for all new functionality
- Focus on backwards compatibility for existing command-line options
- Prioritize performance and memory efficiency throughout implementation
- Ensure cross-platform compatibility (Linux, macOS, Windows)

## Documentation Plan

1. Complete missing README.md sections
   - Key Features section
   - Complete Design Philosophy section
   - Development setup instructions

2. Create developer documentation
   - Architecture overview with diagrams
   - Key algorithms explanation
   - Performance considerations

3. Build comprehensive usage documentation
   - Command-line option reference
   - Advanced usage examples
   - Integration with other tools