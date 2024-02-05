mod commands;

use std::net::TcpStream;
use std::io::prelude::*;
use ssh2::Session;
use std::path::PathBuf;
use crate::commands::{commands_factory, CommandTypes};
use crate::commands::enums::Commands;

fn main() {
    // let tcp = TcpStream::connect("0.0.0.0:2222").unwrap();
    // let mut sess = Session::new().unwrap();
    //
    // sess.set_tcp_stream(tcp);
    // sess.handshake().unwrap();
    //
    // sess.userauth_password("foo", "pass").unwrap();
    //
    //
    // let mut path = PathBuf::new();
    // path.push( "/home");
    // path.push( "foo/");
    // println!("{:?}",&path);
    // let mut sftp = sess.sftp().unwrap();
    //
    //
    //
    // match sftp.readdir(&path) {
    //
    //     Ok(dir_entries) => {
    //         for entry in dir_entries {
    //            println!("{:?}",entry);
    //          //  println!("{:?}",entry.1.size);
    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("Failed to list directory: {:?}", err);
    //         return;
    //     }
    // }


    let commands = commands_factory("/dire",Commands::ONDIRECTORYLISTCOMMAND);
    println!("{:?}",commands);
    match  commands {
        CommandTypes::OnDirectoryListCommand(item) =>{
            println!("{}",item.super_struct.command);
            println!("{}",item.super_struct.dir_or_file);
        }
        CommandTypes::OnFileReadCommand(item) => {
            println!("{}",item.super_struct.command);
            println!("{}",item.super_struct.dir_or_file);
        }
    }
}
