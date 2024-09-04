use nwslib::{forecast::forecast::get_current_condtions, grid_json::grid_json::RelativeLocationProperties};

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    lat: Option<f32>,
    long: Option<f32>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,


    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    CurrentConditions {
        lat: f32,
        long: f32,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(lat) = cli.lat {
        if let Some(long) = cli.long {
            println!("{}", get_current_condtions(lat,long).await.expect("Could not get forecast!"));
            
        }
    }
    
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::CurrentConditions { lat, long } ) => {
            println!("{}", get_current_condtions(*lat,*long).await.expect("Could not get forecast!"));            
        }
        None => {}
    }

    // Continued program logic goes here...
}
