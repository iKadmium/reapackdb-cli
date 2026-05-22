mod commands;
mod manifest;
mod reapack;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "reapackdb")]
#[command(about = "Declarative ReaPack package manager")]
struct Cli {
    #[arg(long, help = "Path to manifest file")]
    manifest: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add package to manifest")]
    Add {
        remote: String,
        category: String,
        package: String,
    },
    #[command(about = "Remove package from manifest")]
    Remove {
        remote: String,
        category: String,
        package: String,
    },
    #[command(about = "Discover packages from configured repos (reapack.ini)")]
    Discover {
        #[arg(long, help = "Path to reapack.ini")]
        ini: Option<PathBuf>,
    },
    #[command(about = "Import installed packages from DB to manifest")]
    Import {
        #[arg(long, help = "Path to ReaPack database")]
        db: Option<PathBuf>,
    },
    #[command(about = "Sync manifest to ReaPack database")]
    Sync {
        #[arg(long, help = "Path to ReaPack database")]
        db: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let manifest_path = manifest::get_manifest_path(cli.manifest)?;

    match cli.command {
        Some(Commands::Add { remote, category, package }) => {
            commands::add_package(&manifest_path, remote, category, package)?;
        }
        Some(Commands::Remove { remote, category, package }) => {
            commands::remove_package(&manifest_path, remote, category, package)?;
        }
        Some(Commands::Discover { ini }) => {
            commands::discover(&manifest_path, ini)?;
        }
        Some(Commands::Import { db }) => {
            commands::import(&manifest_path, db)?;
        }
        Some(Commands::Sync { db }) => {
            commands::sync(&manifest_path, db)?;
        }
        None => {
            println!("No command specified. Use --help for usage.");
        }
    }

    Ok(())
}
