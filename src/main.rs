mod commands;
mod watchly;

use clap::Parser;
use commands::Commands;
use watchly::Watchly;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init{ client_name, client_address, hourly_rate } => Watchly::initialize(client_name, client_address, hourly_rate).map_or_else(|_| {
            eprintln!("Cannot initialize Watchly, maybe watchly is already initialized");
        }, |_| {
            println!("New Watchly project initialized");
        })
    }
}
