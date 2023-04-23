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
    Get { key: String },

    #[command(name = "set")]
    #[command(about = "Set value for specific key")]
    Set { key: String, value: String },

    #[command(name = "rm")]
    #[command(about = "Remove value for specific key")]
    Remove { key: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { key: _ } => {
            panic!("unimplemented")
        }
        Commands::Remove { key: _ } => {
            panic!("unimplemented")
        }
        Commands::Set { key: _, value: _ } => {
            panic!("unimplemented")
        }
    };
    // Continued program logic goes here...
}
