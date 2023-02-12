use std::collections::HashMap;
use std::process::Command;

use crate::utils::read_file_to_hashmap;

pub fn run (file_path_str: &String, editor: &String, mark: &str) -> () {
    let marks: HashMap<String, String> = read_file_to_hashmap(&file_path_str);

    let mark_path = match marks.get(&mark.to_string()) {
        None => {
            println!("Mark not found");
            std::process::exit(0);
        },
        val => val.unwrap(),
    };

    let _command = Command::new(&editor)
        .arg(&mark_path)
        .status()
        .expect("Something went wrong");
}
