use std::collections::HashMap;

use clap::{Parser, Subcommand};

use crate::delete::delete_mark;
use crate::list::list;
use crate::set::set;
use crate::types::ConfigType;

mod config;
mod delete;
mod list;
mod set;
mod types;
mod utils;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    mark: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Set { alias: String, source: String },
    Delete { alias: String },
}

fn main() {
    let cli = Cli::parse();
    let files: HashMap<ConfigType, String> = config::initialize();

    if let Some(mark) = cli.mark.as_deref() {
        println!("value for mark: {}", mark);
        std::process::exit(0)
    }

    // if let Some(mark_to_delete) = cli.delete.as_deref() {
    //     let file_path_str: &String = &files[&ConfigType::Marks];
    //     delete_mark(mark_to_delete, &file_path_str);
    //     std::process::exit(0)
    // }

    match &cli.command {
        Some(Commands::Delete { alias }) => {
            let file_path_str: &String = &files[&ConfigType::Marks];
            delete_mark(&file_path_str, &alias);
        }
        Some(Commands::List) => {
            let file_path_str: &String = &files[&ConfigType::Marks];
            list(&file_path_str);
        }
        Some(Commands::Set { alias, source }) => {
            let file_path_str: &String = &files[&ConfigType::Marks];
            set(&alias, &source, &file_path_str);
        }
        None => {
            println!("default");
        }
    }
}
