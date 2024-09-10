// nws-cli, a CLI application that gets the forecast or current conditions from the National Weather Service
//     Copyright (C) 2024 Margaret Joan Miller

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.

//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

use nwslib::forecast::forecast::{get_current_condtions, get_five_day_forecast};

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None , arg_required_else_help = true)]
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
    CurrentConditions {
        lat: f32,
        long: f32,
    },

    FiveDayForecast {
        lat: f32,
        long: f32
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
        Some(Commands::CurrentConditions { lat, long } ) => {
            println!("{}", get_current_condtions(*lat,*long).await.expect("Could not get forecast!"));            
        }
        Some(Commands::FiveDayForecast { lat, long }) => {
            println!("{}", get_five_day_forecast(*lat,*long).await.expect("Could not get forecast!"))
        }
        None => (),
    }

}


