use std::net::TcpStream;
use std::path::{Path, PathBuf};
use ssh2::Session;
use crate::types::sftp_config::SftpConfig;

pub fn move_file(sftp_config: &SftpConfig,file_name:String){
    let connection_string = format!("{}:{}", sftp_config.ip, sftp_config.port);

    let tcp = TcpStream::connect(connection_string).unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&sftp_config.username, &sftp_config.password).unwrap();
    let  sftp = sess.sftp().unwrap();

    let binding = file_name.clone();
    let file_name_from_path = Path::new(&binding);

    let mut path = PathBuf::new();
    path.push(&sftp_config.directory);
    path.push( file_name_from_path.file_name().unwrap());
  
   
    let mut target_path = PathBuf::new();
    target_path.push(&sftp_config.processed_directory);
   // sftp.opendir(&target_path).unwrap();
   // target_path.push("processed");   
   if let Ok(stat) = sftp.stat(&target_path) {
    if stat.is_dir() {
        // Directory exists, no need to create
        println!("Directory already exists: {}", &sftp_config.processed_directory);
       
    }
} else {

    // Directory doesn't exist, create it
    match sftp.mkdir(&target_path, 0o755) {
        Ok(()) => {
            println!("Directory created: {}",  &sftp_config.processed_directory);
           
        }
        Err(err) => {
            eprintln!("Failed to create directory: {:?}", err);
           
        }
     }
    }

 
   target_path.push(file_name_from_path.file_name().unwrap());

   sftp.rename(&path, &target_path,None).unwrap();
}