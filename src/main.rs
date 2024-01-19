use std::fmt::format;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()>{
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file: `{:?}`", &args.path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
