//! Display module handles the formatting and output of directory trees
mod colors;
mod format;
mod state;
mod utils;

#[cfg(test)]
mod tests;

pub use colors::should_use_colors;
pub use format::format_tree;
