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
    #[command(subcommand)]
    Config(ConfigCommand),

    /// Generate an index document
    Index,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Get a config value
    Get {
        /// The key to get (e.g., docs.root, rules.max_lines)
        key: String,
    },

    /// Set a config value
    Set {
        /// The key to set (e.g., docs.root, rules.max_lines)
        key: String,
        /// The value to set
        value: String,
    },

    /// List all config values
    List,

    /// Print path to config file
    Path,
}
