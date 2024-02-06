
use crate::events::directory_listed_event::DirectoryListedEvent;
use crate::types::sftp_config::SftpConfig;


use std::net::TcpStream;

use ssh2::Session;
use std::path::PathBuf;


#[derive(Debug)]
pub struct OnDirectoryListCommand {
    pub directory_path:String,


}


impl OnDirectoryListCommand {
  
    pub fn apply(&self, sftp_config: &SftpConfig) -> Result<DirectoryListedEvent, ssh2::Error> {
        let connection_string = format!("{}:{}", sftp_config.ip, sftp_config.port);

        let tcp = TcpStream::connect(connection_string).unwrap();
        let mut sess = Session::new().unwrap();
        
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        
        sess.userauth_password(&sftp_config.username, &sftp_config.password).unwrap();
        
        
        let mut path = PathBuf::new();
        path.push( &sftp_config.directory);
        
   
        let  sftp = sess.sftp().unwrap();
        
        let mut directory_listed_event = DirectoryListedEvent{directory :sftp_config.directory.clone(),files: Vec::new(),};
        
        match sftp.readdir(&path) {
        
            Ok(dir_entries) => {
                for entry in dir_entries {
                    let file_name = entry.0
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_owned()).unwrap_or_default();
                let is_directory = entry.1.is_dir();
                if !is_directory {
                     directory_listed_event.files.push(file_name);
                }
                }
            }
            Err(err) => {
                eprintln!("Failed to list directory: {:?}", err);
                return Err(err);
            }
        }
        Ok(directory_listed_event)

    }
}
