mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        _ => println!("Hello World")
    }
}
