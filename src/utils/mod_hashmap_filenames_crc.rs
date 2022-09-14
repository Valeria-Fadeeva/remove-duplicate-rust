use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate walkdir;
use walkdir::WalkDir;

use permissions::*;

extern crate crc32fast;

pub fn hashmap_filenames_crc(root_dir: &str, mut filenames: HashMap<String, String>) -> HashMap<String, String> {
    let mut file_count: u64 = 0;

    for entry in WalkDir::new(root_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && !e.file_type().is_symlink() && is_readable(e.path().as_os_str()).unwrap())
    {
        file_count += 1;
        let mut count: usize = 0;
        let mut buffer = [0; 65536];

        let f_name = String::from(entry.path().to_string_lossy());

        println!("{}", f_name);

        let metadata = fs::metadata(&f_name).unwrap();

        if metadata.len() > 0 && Path::new(&f_name).is_file() {
            let f = File::open(f_name.as_str());

            let status = match f {
                Ok(f) => Ok(f),
                Err(e) => Err(e),
            };

            if status.is_ok() {
                println!("Reading bytes");
                for byte in status.unwrap().bytes() {
                    if count < 65536 {
                        buffer[count] = byte.unwrap();
                        count += 1;
                    } else {
                        break;
                    }
                }
            } else {
                eprintln!("Filepath {} Error {:#?}", f_name, status.err());
                buffer[0] = u8::from(0);
            }
        } else {
            buffer[0] = u8::from(0);
        }

        println!("Making checksum");
        let checksum = crc32fast::hash(&buffer);

        println!("Insert path and checksum into HashMap");
        filenames
            .entry(f_name.clone().replace(root_dir, ""))
            .or_insert(format!("{:X}", checksum));

        println!("{:<10} {:>10} {}", file_count, &checksum, &f_name);
        println!();
    }

    filenames
}
