use std::{fs, io::{Read, Write}};
use serde::{Deserialize, Serialize};


/// The struct for reading and writing backend mapping.
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Routes {
    pub mapping: Vec<BackendMapping>,
    pub path: String
}
/// The struct for defining the IP:Port address and its assigned path.
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct BackendMapping {
    pub addr: String,
    pub path: String
}

impl Routes {

    pub fn new(path: String) -> Self {
        Self {
            path,
            mapping: Vec::new()
        }
    }

    pub fn read(&self) -> Vec<BackendMapping> {
        if let Ok(mut f) = fs::OpenOptions::new().read(true).open(&self.path) {
            let mut contents: String = String::new();
            if let Ok(_) = f.read_to_string(&mut contents) {
                match serde_json::from_str(contents.as_str()) {
                    Ok(data) => {
                        let data: Vec<BackendMapping> = data;
                        return data;
                    }
                    Err(error) => panic!("BACKEND_MAPPING_READ_FAILED: {:?}",error)
                }
            }
        }
        Vec::new()
    }

    pub fn write(&self, data: Vec<BackendMapping>) -> bool {
        match serde_json::to_string_pretty(&data) {
            Ok(contents) => {
                match fs::OpenOptions::new().create(true).truncate(true).write(true).open(&self.path) {
                    Ok(mut f) => {
                        if let Err(error) = f.write_all(contents.as_bytes()) {
                            panic!("BACKEND_MAPPING_WRITE_FAILED: {:?}",error)
                        }
                    }
                    Err(error) => panic!("BACKEND_MAPPING_FILE_CREATE_FAILED: {:?}",error)
                }
            }
            Err(error) => panic!("BACKEND_MAPPING_DESERIALIZE_FAILED: {:?}",error)
        }
        true
    }
}
