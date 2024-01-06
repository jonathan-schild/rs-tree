use clap::{Parser, Subcommand};
use rs_tree::utility::hash_password;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// generates new v4 uuid
    Uuid {
        /// generates a all zero uuid
        #[arg(short, long, default_value = "false")]
        nil: bool,
    },
    /// creates a hash of `password`
    Hash { password: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Uuid { nil } => {
            if nil {
                println!("{}", Uuid::nil())
            } else {
                println!("{}", Uuid::new_v4())
            }
        }
        Commands::Hash { password } => println!(
            "{}",
            hash_password(&password).expect("an unexpected error occurred")
        ),
    }
}
