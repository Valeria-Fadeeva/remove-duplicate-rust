use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::exit;

mod utils;
use utils::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let new_dir: &str;
    let old_dir: &str;

    if args.len() == 3 {
        new_dir = (&args[1]).as_str();
        old_dir = (&args[2]).as_str();
    } else {
        new_dir = "./test1";
        old_dir = "./test2";
    }

    if Path::new(new_dir).is_dir() == false {
        println!("Path {} doesn't exist", &new_dir);
        exit(1);
    }

    if Path::new(old_dir).is_dir() == false {
        println!("Path {} doesn't exist", &old_dir);
        exit(1);
    }

    let mut filenames1:HashMap<String, String> = HashMap::new();
    let mut filenames2:HashMap<String, String> = HashMap::new();

    println!("Walking {}", &new_dir);
    filenames1 = mod_hashmap_filenames_crc::hashmap_filenames_crc(&new_dir, filenames1);

    println!("Walking {}", &old_dir);
    filenames2 = mod_hashmap_filenames_crc::hashmap_filenames_crc(&old_dir, filenames2);

    mod_remove_duplicate::remove_duplicate(&old_dir, filenames1, filenames2).unwrap();
    mod_remove_empty_folders::remove_empty_folders(&old_dir).unwrap();

}
