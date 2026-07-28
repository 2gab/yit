mod auth;
mod commands;
mod config;
mod db;
mod youtube;

use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "yit", about = "git for YouTube", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Authenticate with Google
    Login,
    /// Start tracking a playlist
    Add { url: String },
    /// List tracked playlists
    Ls,
    /// Sync tracks from YouTube
    Fetch,
    /// Download pending tracks
    Pull,
    /// Show library status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let data_dir = dirs::data_dir()
        .expect("could not find data directory")
        .join("yit");

    fs::create_dir_all(&data_dir)?;

    let cfg = config::load()?;
    let db_path = format!("sqlite:{}", data_dir.join("yit.db").display());
    let pool = db::connect(&db_path).await?;

    let cli = Cli::parse();

    match cli.command {
        Command::Login => {
            auth::login(&cfg.google).await?;
        }
        Command::Add { url } => {
            commands::playlist::add(&pool, &url).await?;
        }
        Command::Ls => {
            commands::playlist::ls(&pool).await?;
        }
        Command::Fetch => {
            commands::fetch::run(&pool).await?;
        }
        Command::Pull => {
            commands::pull::run(&pool, &cfg).await?;
        }
        Command::Status => println!("status: not implemented yet"),
    }

    pool.close().await;
    Ok(())
}
