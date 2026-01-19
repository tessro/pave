use clap::{Parser, Subcommand};

/// PAVED documentation tool - structured docs optimized for AI agents
#[derive(Parser)]
#[command(name = "paver")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize a project with PAVED documentation
    Init,

    /// Validate PAVED documentation
    Check,

    /// Create a new document from template
    New,

    /// Generate prompts for AI agents
    Prompt,

    /// Manage git hooks for documentation validation
    Hooks,

    /// View or modify paver configuration
    Config,

    /// Generate an index document
    Index,
}
