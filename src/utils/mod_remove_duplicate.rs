use std::collections::HashMap;
use std::fs;

pub fn remove_duplicate(root_dir: &str, filenames1: HashMap<String, String>, filenames2: HashMap<String, String>) -> Result<(), ()> {

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
            let path_to_remove: String = format!("{}{}", root_dir, filepath);

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

    //TODO: Сделать возврат Error

    Ok(())
}
