mod commands;
mod config;
mod db;
mod playlist;
mod youtube;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "yit", about = "git for YouTube", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Clone a remote playlist into a new directory
    Clone { url: String },
    /// Turn the current directory into a tracked playlist
    Init { url: String },
    /// Show diff between local and remote playlist state
    Status,
    /// Fetch remote changes and download new tracks
    Sync,
    /// Alias for `sync`
    Pull,
    /// Remove yit tracking from the current directory
    Untrack,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::load()?;
    let cli = Cli::parse();

    match cli.command {
        Command::Clone { url } => commands::clone::run(&url, &cfg).await?,
        Command::Init { url } => commands::init::run(&url).await?,
        Command::Status => commands::status::run().await?,
        Command::Sync | Command::Pull => commands::sync::run(&cfg).await?,
        Command::Untrack => commands::untrack::run()?,
    }

    Ok(())
}
