use anyhow::Result;
use clap::Parser;
use paver::cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            println!("paver init: not yet implemented");
        }
        Command::Check => {
            println!("paver check: not yet implemented");
        }
        Command::New => {
            println!("paver new: not yet implemented");
        }
        Command::Prompt => {
            println!("paver prompt: not yet implemented");
        }
        Command::Hooks => {
            println!("paver hooks: not yet implemented");
        }
        Command::Config => {
            println!("paver config: not yet implemented");
        }
        Command::Index => {
            println!("paver index: not yet implemented");
        }
    }

    Ok(())
}
