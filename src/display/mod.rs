//! Display module handles the formatting and output of directory trees
mod format;
mod state;
mod utils;
mod colors;

#[cfg(test)]
mod tests;

pub use format::format_tree;
pub use colors::should_use_colors;
