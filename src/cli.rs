use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new event to commemorate
    Add {
        /// Name of the event
        #[arg(short, long)]
        name: String,

        /// Description of the event
        #[arg(short, long)]
        description: String,

        /// Time of the event (format: YYYY-MM-DD-HH-MM-SS)
        #[arg(short, long)]
        time: Option<String>,

        /// Timezone of the event (e.g., "UTC", "America/New_York")
        #[arg(short, long, default_value = "UTC")]
        area: String,

        /// Custom path to store the event file
        #[arg(short, long)]
        path: Option<String>,
    },

    /// List all commemorated events
    List,

    /// Show details of a specific event
    Tell {
        /// Name or path of the event
        event_identifier: String,
    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
