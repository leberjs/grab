use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use ron::de::from_reader;
use ron::ser::{to_string_pretty, PrettyConfig};

pub fn read_file_to_hashmap(file_path_str: &String) -> HashMap<String, String> {
    let path = Path::new(&file_path_str);

    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            println!("Failed to load file: {}", err);
            std::process::exit(1);
        }
    };

    let data: HashMap<String, String> = match from_reader(&file) {
        Ok(d) => d,
        Err(err) => {
            println!("Failed to load data from file: {}", err);
            std::process::exit(1);
        }
    };

    data
}

pub fn write_hashmap_to_file(file_path_str: &String, marks: &HashMap<String, String>) -> () {
    let path = Path::new(&file_path_str);

    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(err) => {
            println!("Failed to load marks file: {}", err);
            std::process::exit(1);
        }
    };

    let pretty = PrettyConfig::new().depth_limit(4);

    match file.write_all(to_string_pretty(&marks, pretty).unwrap().as_bytes()) {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to set mark: {}", err);
            std::process::exit(1);
        }
    }
}
