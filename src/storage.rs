use std::{
    fs::{self, File, OpenOptions},
    io::{self, Error, Write},
    path::PathBuf,
};

use serde_json::Value;

use directories::ProjectDirs;

pub fn is_storage_exist() -> bool {
    ProjectDirs::from("com", "chetanxpro", "go")
        .map(|project_path| project_path.config_dir().exists())
        .unwrap_or(false)
}

pub fn create_storage(storage_path: PathBuf) -> Result<(), Error> {
    File::create(storage_path).map(|_| ()).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(getProjectJsonPath().unwrap())
        .unwrap();

    file.write_all(b"{}").unwrap();

    Ok(())
}

pub fn getProjectJsonPath() -> Result<PathBuf, Error> {
    ProjectDirs::from("com", "chetanxpro", "go")
        .map(|path| path.config_dir().join("list.json").to_path_buf())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Project path not found"))
}

pub fn write_to_json(name: String, path: PathBuf) -> Result<(), Error> {
    let mut list_json_data: Value = {
        println!(
            "project path: {}",
            getProjectJsonPath().unwrap().to_string_lossy()
        );
        let text: String = fs::read_to_string(getProjectJsonPath().unwrap()).unwrap();

        serde_json::from_str::<Value>(&text)
    }
    .unwrap();

    // println!("")

    if let Some(object) = list_json_data.as_object_mut() {
        object.insert(String::from(name), Value::from(path.to_str()));
    }

    let updated_json: String = serde_json::to_string_pretty(&list_json_data)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(getProjectJsonPath().unwrap())
        .unwrap();

    file.write_all(updated_json.as_bytes())?;

    Ok(())
}

pub fn remove_from_json(name: String) -> Result<(), Error> {
    let mut list_json_data: Value = {
        let text: String = fs::read_to_string(getProjectJsonPath().unwrap()).unwrap();
        serde_json::from_str::<Value>(&text)
    }
    .unwrap();

    if let Some(object) = list_json_data.as_object_mut() {
        object.remove(&name).unwrap();
    }

    let updated_json: String = serde_json::to_string_pretty(&list_json_data)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(getProjectJsonPath().unwrap())
        .unwrap();

    file.write_all(updated_json.as_bytes())?;

    Ok(())
}

pub fn read_from_json(name: &String) -> Result<PathBuf, Error> {
    let mut list_json_data = {
        let text = fs::read_to_string(getProjectJsonPath().unwrap()).unwrap();
        serde_json::from_str::<Value>(&text)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
    .unwrap();

    if let Some(object) = list_json_data.as_object_mut() {
        let value = object.get(&name.to_owned()).unwrap();

        if let Some(path_str) = value.as_str() {
            Ok(PathBuf::from(path_str))
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Expected a string for the path",
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "name not found in list",
        ))
    }
}

pub fn list_all_names() -> Result<(), Error> {
    let mut list_json_data = {
        let text = fs::read_to_string(getProjectJsonPath().unwrap()).unwrap();
        serde_json::from_str::<Value>(&text)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
    .unwrap();

    if let Some(object) = list_json_data.as_object_mut() {
        let _value = object
            .into_iter()
            .for_each(|val| println!("Name: {}: {}", val.0, val.1));
    }
    Ok(())
}
