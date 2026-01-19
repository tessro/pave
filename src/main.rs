use anyhow::Result;
use clap::Parser;
use paver::cli::{Cli, Command, ConfigCommand};
use paver::commands::config;

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
        Command::Config(cmd) => match cmd {
            ConfigCommand::Get { key } => {
                config::get(&key)?;
            }
            ConfigCommand::Set { key, value } => {
                config::set(&key, &value)?;
            }
            ConfigCommand::List => {
                config::list()?;
            }
            ConfigCommand::Path => {
                config::path()?;
            }
        },
        Command::Index => {
            println!("paver index: not yet implemented");
        }
    }

    Ok(())
}
