use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

mod recharge;

use recharge::models::GameConfig;
use recharge::process::process_game;

#[derive(Parser)]
#[command(name = "l-games")]
#[command(about = "Game tools and calculators", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate game exchange rates from config files
    Calc {
        /// Directory containing game config files
        #[arg(short, long, default_value = "data")]
        data_dir: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Calc { data_dir } => {
            let path = Path::new(data_dir);
            if !path.exists() {
                println!("Data directory '{:?}' not found.", path);
                return Ok(());
            }

            println!("Loading game data from {:?}...\n", path);

            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();

                if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let content = fs::read_to_string(&file_path)?;
                    match serde_json::from_str::<GameConfig>(&content) {
                        Ok(config) => process_game(config),
                        Err(e) => println!("Failed to parse {:?}: {}", file_path, e),
                    }
                }
            }
        }
    }

    Ok(())
}
