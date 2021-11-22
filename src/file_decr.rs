use serde::{Serialize, Deserialize};
use std::{fs::DirEntry, time::SystemTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileDecr {
    name: String,
    file_type: bool,
    last_modify: u128,
    file_size: u64,
}

impl FileDecr {
    pub fn extra_meta(entity: DirEntry) -> FileDecr {
        let name = entity.file_name();
        let meta = entity.metadata().unwrap();
        let file_type = meta.is_file();
        let last_modify = meta
            .modified()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut file_size: u64 = 0;
        if file_type {
            file_size = meta.len();
        }
        FileDecr {
            name: String::from(name.to_str().unwrap()),
            file_type: file_type,
            last_modify: last_modify,
            file_size: file_size,
        }
    }
}
