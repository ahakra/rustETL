use std::io;
use std::{fs::File, io::Read};
use crate::types::sftp_config::SftpConfig;




pub fn read(file_path: String) -> Result<SftpConfig, io::Error> {
     // Read the file into a String
     let mut file_content = String::new();
     let mut file = File::open(&file_path)?;
 
     file.read_to_string(&mut file_content)?;
 
     // Deserialize JSON into SftpConfig struct
     let config: SftpConfig = serde_json::from_str(&file_content)?;
 
     // Return the deserialized configuration
     Ok(config)
 }