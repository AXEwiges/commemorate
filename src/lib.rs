pub mod cli;
pub mod commands;
pub mod error;
pub mod utils;

// Re-export main types and functions for easier use
pub use cli::{parse_cli, Cli, Commands};
pub use commands::{add_event, list_events, tell_event};
pub use error::{CommemorateError, CommemorateResult};
pub use utils::{
    decrypt, encrypt, format_duration, get_memoria_path, hash_password, list_memoria_files,
    parse_time, read_file, verify_password, write_file, TimeInfo,
};

/// The main entry point for the commemorate library.
///
/// This function processes the command line arguments and executes the appropriate command.
pub fn run() -> CommemorateResult<()> {
    let cli = parse_cli();

    match cli.command {
        Commands::Add {
            name,
            description,
            time,
            area,
            path,
        } => add_event(&name, &description, time.as_deref(), &area, path.as_deref()),
        Commands::List => list_events(),
        Commands::Tell { event_identifier } => tell_event(&event_identifier),
    }
}
