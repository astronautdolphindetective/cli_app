use anyhow::{Context, Result};
use log::{info, warn};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file: `{:?}`", &args.path))?;

    
    cli_app::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
