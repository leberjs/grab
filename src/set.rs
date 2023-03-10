use std::collections::HashMap;

use crate::utils::{read_file_to_hashmap, write_hashmap_to_file};

pub fn set(alias: &String, source: &String, file_path_str: &String) -> () {
    let mut marks: HashMap<String, String> = read_file_to_hashmap(&file_path_str);

    marks.insert(alias.to_string(), source.to_string());

    write_hashmap_to_file(&file_path_str, &marks)
}
