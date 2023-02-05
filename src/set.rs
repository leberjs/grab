use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use ron::de::from_reader;
use ron::ser::{PrettyConfig, to_string_pretty};

pub fn set(alias: &String, source: &String, file_path_str: &String) -> () {
    let path = Path::new(&file_path_str);

    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            println!("Failed to load marks file: {}", err);
            std::process::exit(1);
        }
    }; 

    let mut marks: HashMap<String, String> = match from_reader(&file) {
        Ok(mrks) => mrks,
        Err(err) => {
            println!("Failed to load marks from file: {}", err);
            std::process::exit(1);
        },
    };

    marks.insert(alias.to_string(), source.to_string());

    drop(file);

    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(err) => {
            println!("Failed to load marks file: {}", err);
            std::process::exit(1);
        },
    };

    let pretty = PrettyConfig::new()
        .depth_limit(4);

    match file.write_all(to_string_pretty(&marks, pretty).unwrap().as_bytes()) {
        Ok(_) => (), 
        Err(err) => {
            println!("Failed to set mark: {}", err);
            std::process::exit(1);
        },
    }
}

