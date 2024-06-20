use std::{
    fs,
    io::{self, Error},
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::storage;

pub fn add_to_file(name: &str, path: &str) {
    if !storage::is_storage_exist() {
        if let Some(project_path) = ProjectDirs::from("com", "chetanxpro", "go") {
            if !project_path.config_dir().exists() {
                fs::create_dir_all(project_path.config_dir())
                    .expect("Error while creating config directory");
            }
            let config_path = project_path.config_dir().join("list.json");
            let _ = storage::create_storage(config_path).expect("Error while creating list");
        }
    }

    if PathBuf::from(path.to_string()).try_exists().unwrap() {
        // println!("Found value: {}", path.display());
    } else {
        eprintln!("Invalid path: {}", path);
        return;
    }

    storage::write_to_json(name.to_string(), PathBuf::from(path.to_string())).unwrap();
}

pub fn read_from_file(name: String) -> Result<PathBuf, Error> {
    if !storage::is_storage_exist() {
        if let Some(project_path) = ProjectDirs::from("com", "chetanxpro", "go") {
            if !project_path.config_dir().exists() {
                fs::create_dir_all(project_path.config_dir())
                    .expect("Error while creating config directory");
            }
            let config_path = project_path.config_dir().join("list.json");
            let _ = storage::create_storage(config_path).expect("Error while creating list");
        }
    }

    // if let Some(path) = storage::read_from_json(name) {
    // } else {
    //     io::Error::new(io::ErrorKind::NotFound, "Not found")
    // }

    let path = storage::read_from_json(&name).unwrap();

    if path.try_exists().unwrap() {
        // println!("Found value: {}", path.display());
        Ok(path)
    } else {
        Err(Error::new(io::ErrorKind::InvalidData, "Not a valid path"))
    }
}
