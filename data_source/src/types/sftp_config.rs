
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct SftpConfig {
    ip: String,
    port: u16,
    username: String,
    password: String,
    directory: String,
    name_regex: String,
    processed_directory:String,
    invalid_directory: String,
    partially_invalid_directory: String,
}
