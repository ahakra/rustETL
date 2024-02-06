pub mod commands;
pub mod types;
pub mod helpers;
pub mod events;
pub mod traits;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write ;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::io;
use tokio;
use commands::{on_directory_list_command::OnDirectoryListCommand, on_file_read_command::OnFileReadCommand, on_records_map_command::OnRecordMapCommand};
use crate::traits::event_trait::StorableEvent;

fn append_to_event_store(event: impl StorableEvent) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("event_store.json")?;

    writeln!(file, "{}", event.serialize())?;

    Ok(())
}
#[tokio::main]
async  fn main() {
    let _ = helpers::crud_service::delete().await;
    let _ = helpers::crud_service::register().await;

      
    // Spawn a new Tokio task for updating health periodically
    let _handle = tokio::spawn(async {
        loop {
            match helpers::crud_service::update_health().await {
                Ok(response) => {
                    println!("Update health Response: {:?}", response.status());
                }
                Err(err) => {
                    eprintln!("Request error for update: {:?}", err);
                }
            }
           
            sleep(Duration::from_secs(10));
        }
    });

    loop {
   
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
                let event = directory_list_command.apply(&sftp_config);

                let arc_event = Arc::new(event.as_ref().unwrap());
                if let Err(err) = append_to_event_store((*arc_event).clone()) {
                    eprintln!("Error storing event: {}", err);
                }

                let files_present: Vec<String> = event
                .map(|event| {
                    event.files.iter()
                        .map(|entry| format!("{}/{}", &sftp_config.directory, entry))
                        .collect()

                })
                .unwrap_or_else(|err| {
                    eprintln!("Error handling DirectoryListedEvent: {}", err);
                    Vec::new() 
                });

                if files_present.len()>0 {
                println!("Found {} files",files_present.len());
                //looping through files to get its content
                for path in &files_present {
                
                    let file_read_command = OnFileReadCommand{file_name:path.to_string()};
                
                    let file_read_event = file_read_command.apply(&sftp_config);

                   
                    file_read_event.map(|event| {
                        

                        let mut record_map_command = OnRecordMapCommand{file_name : event.file_name, file_content: event.file_content};

                        let record_mapped_event = record_map_command.apply() ;

                        let arc_event = Arc::new(record_mapped_event.as_ref().unwrap());
                        if let Err(err) = append_to_event_store((*arc_event).clone()) {
                            eprintln!("Error storing event: {}", err);
                        }
                        record_mapped_event.map(|event| {
                            helpers::move_file::move_file(&sftp_config,event.file_name);
                        }).unwrap_or_else(|err| {
                            eprintln!("Error parsing file content: {}", err);
                        });

                    }) .unwrap_or_else(|err| {
                        eprintln!("Error reading file content: {}", err);
                    
                    });
                }
                
            } else{
                println!("No files in directory");
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    }

    }

