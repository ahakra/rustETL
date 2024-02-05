use std::net::TcpStream;
use std::path::PathBuf;
use ssh2::Session;
use crate::types::sftp_config::SftpConfig;

pub fn moveFile(sftp_config: &SftpConfig,file_name:String){
    let connection_string = format!("{}:{}", sftp_config.ip, sftp_config.port);

    let tcp = TcpStream::connect(connection_string).unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&sftp_config.username, &sftp_config.password).unwrap();


    let mut path = PathBuf::new();
    path.push( file_name);

    let mut processed_path =PathBuf::new();
    processed_path.push(&sftp_config.processed_directory);



    let mut sftp = sess.sftp().unwrap();
    // Check if the directory exists, create it if not

    let mut target_path = PathBuf::new();
    target_path.push("/sftpFiles");



    // // Check if the directory exists, create it if not
    // if let Err(err) = sftp.stat(&target_path) {
    //     if let Err(create_err) = sftp.mkdir(&target_path, 0o755) {
    //         eprintln!("Error creating directory: {:?}", create_err);
    //         return;
    //     }
    //     println!("Created directory: {:?}", target_path);
    // }
sftp.open(&target_path).unwrap();

    let mut sftp = sess.sftp().unwrap();
    // Check if the directory exists, create it if not

    let mut target_path = PathBuf::new();
    target_path.push("sftpFiles/processed");
    sftp.mkdir(&target_path, 755)?;


         //sftp.rename(&path, &processed_path,None).unwrap();
}