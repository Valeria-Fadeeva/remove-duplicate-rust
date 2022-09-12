extern crate walkdir;
use walkdir::WalkDir;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::env;
use std::process::exit;

extern crate checksum;
use checksum::crc::Crc as crc;


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

    let mut filenames1 = HashMap::new();
    let mut filenames2 = HashMap::new();

    for entry in WalkDir::new(new_dir).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.path().to_string_lossy());
        let mut crc = crc::new(f_name.as_str());

        let crc32 = crc.checksum().expect("error").crc32;

        filenames1.entry(f_name.clone().replace(new_dir, "")).or_insert(format!("{:X}", crc32));
    }

    for entry in WalkDir::new(old_dir).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.path().to_string_lossy());
        let mut crc = crc::new(f_name.as_str());

        let crc32 = crc.checksum().expect("error").crc32;

        filenames2.entry(f_name.clone().replace(old_dir, "")).or_insert(format!("{:X}", crc32));
    }

    for file1 in filenames1 {
        let filepath = file1.0.clone();
        let id = file1.0.as_str();
        let hash1 = file1.1;
        let hash2 = if filenames2.contains_key(id) {
            String::from(filenames2.get(id).unwrap())
        } else {
            String::from("")
        };

        if hash1 == hash2 {
            let path_to_remove: String = format!("{}{}", old_dir, filepath);

            let status = match fs::remove_file(&path_to_remove) {
                Ok(f) => Ok(f),
                Err(e) => Err(e),
            };

            if status.is_ok() {
                println!("Duplicate deleted: {}", &path_to_remove);
            } else {
                println!("Duplicate doesn't deleted: {}", &path_to_remove);
                eprintln!("{:#?}", status.err());
                continue;
            }
        }
    }

    let mut vec: Vec<String> = Vec::new();

    for entry in WalkDir::new(old_dir).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_file()) {
        let f_name = String::from(entry.path().to_string_lossy());

        vec.push(f_name.clone());
    }

    vec.sort();

    while vec.len() > 0 {
        let filepath = vec.pop().unwrap();
        let path_to_remove: String = format!("{}", filepath);

        let status = match fs::remove_dir(&path_to_remove) {
            Ok(f) => Ok(f),
            Err(e) => Err(e),
        };

        if status.is_ok() {
            println!("Folder deleted: {}", &path_to_remove);
        } else {
            println!("Folder doesn't deleted: {}", &path_to_remove);
            eprintln!("{:#?}", status.err());
            continue;
        }
    }

}
