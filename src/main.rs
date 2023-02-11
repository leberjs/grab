use std::collections::HashMap;

use clap::{Parser, Subcommand};

use crate::delete::delete_mark;
use crate::list::list;
use crate::run::run;
use crate::set::set;
use crate::types::ConfigType;
use crate::utils::get_editor_from_config;

mod config;
mod delete;
mod list;
mod run;
mod set;
mod types;
mod utils;

#[derive(Parser)]
struct Cli {
    alias: Option<String>,

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
            let alias = match cli.alias.as_deref() {
                None => {
                    println!("Must provide [ALIAS] for default functionality: `grab [ALIAS]`");
                    std::process::exit(0);
                },
                val => val.unwrap(), 
            };
            let config_file_path_str: &String = &files[&ConfigType::Config];
            let editor: String = get_editor_from_config(&config_file_path_str);

            let marks_file_path_str: &String = &files[&ConfigType::Marks];
            run(&marks_file_path_str, &editor, &alias);
        }
    }
}
