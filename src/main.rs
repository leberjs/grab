use std::collections::HashMap;

use clap::{Parser, Subcommand};

use crate::set::set;
use crate::types::ConfigType;

mod config;
mod set;
mod types;

#[derive(Parser)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Set {
        alias: String,
        source: String,
    }
}

fn main() {
    let cli = Cli::parse();
    let files: HashMap<ConfigType, String> =  config::initialize();

    match &cli.command {
        Commands::List => {
            println!("list triggered");
        },
        Commands::Set {alias, source} => {
            let file_path_str: &String = &files[&ConfigType::Marks];
            set(&alias, &source, &file_path_str);
        }
    }
}

