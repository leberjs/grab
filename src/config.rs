use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Error, Write};
use std::path::Path;

use dirs::config_dir;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::Serialize;

use crate::types::ConfigType;

#[derive(Serialize)]
struct Config {
    marks: String,
    editor: String,
}

pub fn initialize() -> HashMap<ConfigType, String> {
    let defaults = HashMap::from([
        ("dir", "/grab"),
        ("marks", "marks.ron"),
        ("config", "config.ron"),
        ("editor", "nvim"),
    ]);

    let config_path = format!("{}{}", config_dir().unwrap().to_str().unwrap(), defaults.get(&"dir").unwrap());

    let config_file_path_str = format!("{}/{}", config_path, defaults.get(&"config").unwrap());
    let config_file_path = Path::new(&config_file_path_str);
     match File::open(&config_file_path) {
        Ok(_) => (),
        Err(_) => match create_config(&config_path, &config_file_path, &defaults) {
            Ok(_) => (),
            Err(err) => {
                println!("Error creating configuration file {}: {}", config_path, err);
                std::process::exit(1);
            },
        } 
    };

    let marks_file_path_str = format!("{}/{}", config_path, defaults.get(&"marks").unwrap());
    let marks_file_path = Path::new(&marks_file_path_str);

    match File::open(&marks_file_path_str) {
        Ok(_) => (),
        Err(_) => match create_marks(&marks_file_path) {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to create marks file at {}: {}", marks_file_path_str, err);
                std::process::exit(1);
            },
        }
    };

    HashMap::from([
        (ConfigType::Config, config_file_path_str),
        (ConfigType::Marks, marks_file_path_str),
    ])
}

fn create_config(config_path: &String, file_path: &Path, defaults: &HashMap<&str, &str>) -> Result<(), Error> {
    create_dir_all(config_path)?;

    let mut file = File::create(file_path)?;

    let config = Config {
        marks: defaults.get(&"marks").unwrap().to_string(),
        editor: defaults.get(&"editor").unwrap().to_string(),
    };

    let pretty = PrettyConfig::new()
        .depth_limit(4);

    file.write_all(to_string_pretty(&config, pretty).unwrap().as_bytes())?;

    Ok(())
}

fn create_marks(file_path: &Path) -> Result<(), Error> {
    let mut file = File::create(file_path)?;

    let marks: HashMap<String, String> = HashMap::new();

    let pretty = PrettyConfig::new()
        .depth_limit(4);

    file.write_all(to_string_pretty(&marks, pretty).unwrap().as_bytes())?;

    Ok(())
}
