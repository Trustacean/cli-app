use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file: {}", args.path.display()))?;

    let stdout = io::stdout();
    let pb = indicatif::ProgressBar::new(content.lines().count() as u64);
    let mut handle = io::BufWriter::new(stdout);

    info!(
        "Searching for pattern: {} in file: {}",
        args.pattern,
        args.path.display()
    );
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line).with_context(|| "Could not write to stdout")?;
            pb.inc(1);
        }
    }
    pb.finish_with_message("Done");
    Ok(())
}
