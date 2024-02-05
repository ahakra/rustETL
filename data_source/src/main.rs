pub mod commands;
pub mod types;
pub mod helpers;
pub mod events;

use commands::{on_directory_list_command::OnDirectoryListCommand, on_file_read_command};



fn main() {

    let sftp_config_result = helpers::read_sftp_config::read("sftp_config.json".to_owned());

    let sftp_config = sftp_config_result.unwrap_or_else(|err| {
        eprintln!("Error reading SFTP config: {}", err);
        std::process::exit(1);
    });

   //initialize list directory command
    let directory_list_command = OnDirectoryListCommand {
        directory_path: sftp_config.directory.clone(),
    };
      
    // Apply the command to collect event which is list of files
    let files_present: Vec<String> = directory_list_command.apply(&sftp_config)
      .map(|event| {
          event.files.iter()
              .map(|entry| format!("{}/{}", &sftp_config.directory, entry))
              .collect()
      })
      .unwrap_or_else(|err| {
          eprintln!("Error handling DirectoryListedEvent: {}", err);
          Vec::new() 
      });
    

    //looping through files to get its content
    for path in &files_present {
      
        let file_read_command = on_file_read_command::OnFileReadCommand{file_name:path.to_string()};
       
        let file_read_event = file_read_command.apply(&sftp_config).map(|event| {
            println!("{:?}",event.file_name);
            println!("{:?}", String::from_utf8_lossy(&event.file_content));
        }) .unwrap_or_else(|err| {
            eprintln!("Error reading file content: {}", err);
           
        });
    }
        


}
