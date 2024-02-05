
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct SftpConfig {
    pub ip: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub directory: String,
    pub name_regex: String,
    pub processed_directory:String,
    pub invalid_directory: String,
    pub partially_invalid_directory: String,
}
