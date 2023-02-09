use std::collections::HashMap;

use crate::utils::{read_file_to_hashmap, write_hashmap_to_file};

pub fn delete_mark(file_path_str: &String, mark: &str,) -> () {
    let mut marks: HashMap<String, String> = read_file_to_hashmap(&file_path_str);

    marks.remove(&mark.to_string());

    write_hashmap_to_file(&file_path_str, &marks)
}
