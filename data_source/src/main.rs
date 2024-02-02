use std::net::TcpStream;
use std::io::prelude::*;
use ssh2::Session;
use std::path::PathBuf;

fn main() {
    let tcp = TcpStream::connect("0.0.0.0:2222").unwrap();
    let mut sess = Session::new().unwrap();
   
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    
    sess.userauth_password("foo", "pass").unwrap();
   
   
    let mut sftp = sess.sftp().unwrap();
   
    let mut path = PathBuf::new();
    path.push( "sftpFiles");


    match sftp.readdir(&path) {
        Ok(dir_entries) => {
            for entry in dir_entries {
               println!("{:?}",entry.0);
               println!("{:?}",entry.1.size);
            }
        }
        Err(err) => {
            eprintln!("Failed to list directory: {:?}", err);
            return;
        }
    }
}
