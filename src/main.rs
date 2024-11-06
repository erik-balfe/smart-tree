use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

/// A smarter tree command that folds large and hidden directories
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to display
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum number of files to show in a directory before folding
    #[arg(long, default_value_t = 20)]
    limit: usize,

    /// Additional arguments to pass to tree command
    #[arg(last = true)]
    tree_args: Vec<String>,
}

fn run_tree(path: &PathBuf, args: &[String]) -> Result<String> {
    let output = Command::new("tree")
        .arg(path)
        .args(args)
        .output()
        .context("Failed to execute tree command")?;

    String::from_utf8(output.stdout).context("Failed to parse tree output")
}

fn main() -> Result<()> {
    let args = Args::parse();

    // For now, just run tree and print output
    let output = run_tree(&args.path, &args.tree_args)?;
    println!("{}", output);

    Ok(())
}
