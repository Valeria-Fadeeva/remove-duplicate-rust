extern crate walkdir;
use walkdir::WalkDir;

use std::fs;

pub fn remove_empty_folders(root_dir: &str) -> Result<(), ()>{
    let mut vec: Vec<String> = Vec::new();

    for entry in WalkDir::new(root_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
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

    //TODO: Сделать возврат Error

    Ok(())
}
