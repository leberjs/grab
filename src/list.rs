use std::collections::HashMap;

use colorized::*;

use crate::utils::read_file_to_hashmap;

pub fn list(file_path_str: &String) -> () {
    let marks: HashMap<String, String> = read_file_to_hashmap(&file_path_str);

    println!("");
    colorize_println("Current Marks:", Colors::BrightGreenFg);
    colorize_println("--------------", Colors::BrightGreenFg);
    println!("");
    for (k, v) in marks {
        println!("* {} - {}", k.color(Colors::RedFg), v.color(Colors::BrightBlueFg));
    }
}
