use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "get")]
    #[command(about = "Get value from specific key")]
    GetCmd { key: String },

    #[command(name = "set")]
    #[command(about = "Set value for specific key")]
    SetCmd { key: String, value: String },

    #[command(name = "rm")]
    #[command(about = "Remove value for specific key")]
    RmCmd { key: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GetCmd { key } => {
            panic!("unimplemented")
        }
        Commands::RmCmd { key } => {
            panic!("unimplemented")
        }
        Commands::SetCmd { key, value } => {
            panic!("unimplemented")
        }
    };
    // Continued program logic goes here...
}
