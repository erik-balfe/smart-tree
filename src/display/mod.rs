//! Display module handles the formatting and output of directory trees
mod format;
mod state;
mod utils;

#[cfg(test)]
mod tests;

pub use format::format_tree;
