use clap::{Parser, Subcommand};

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
    let cli = Cli::parse();

    match cli.command {
        Command::Login => println!("login: not implemented yet"),
        Command::Add { url } => println!("add: {url}"),
        Command::Ls => println!("ls: not implemented yet"),
        Command::Fetch => println!("fetch: not implemented yet"),
        Command::Pull => println!("pull: not implemented yet"),
        Command::Status => println!("status: not implemented yet"),
    }

    Ok(())
}
