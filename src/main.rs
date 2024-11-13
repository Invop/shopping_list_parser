use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "shopping_list_parser",
    about = "Shopping-list parser for edu purposes",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Parse a shopping list from a file")]
    Parse {
        #[arg(short, long, value_name = "FILE", help = "Path to the shopping list file to be parsed")]
        file: PathBuf,
        #[arg(short, long, help = "Enable verbose mode for detailed output")]
        verbose: bool,
    },
    Credits,
}

fn parse_file(path: &PathBuf, verbose: bool) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    if verbose {
        println!("Parsing file: {}", path.display());
    }

    shopping_list_parser::parse_shopping_list(&content)
        .with_context(|| "Failed to parse shopping list")?;

    Ok(())
}

fn show_credits() {
    println!("Shopping List Parser");
    println!("Created by: Andrii Shandryk");
    println!("License: MIT");
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file, verbose } => {
            parse_file(&file, verbose)?;
        }
        Commands::Credits => {
            show_credits();
        }
    }

    Ok(())
}
