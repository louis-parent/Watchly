use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize Watchly project
    Init {
        /// Client name of the current project
        #[arg(long)]
        client_name: Option<String>,
        
        /// Client address of the current project
        #[arg(long)]
        client_address: Option<String>,
        
        /// Default hourly rate to be apply in this project
        #[arg(long)]
        hourly_rate: Option<f64>
    },
}
