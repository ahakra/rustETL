pub mod enums;
pub mod structs;
use enums::Commands;
use structs::on_file_read_command::OnFileReadCommand;
use structs::on_directory_list_command::OnDirectoryListCommand;
use crate::commands::enums::Commands::ONFILEREADCOMMAND;

#[derive(Debug)]
pub enum CommandTypes {
    OnFileReadCommand(OnFileReadCommand),
    OnDirectoryListCommand(OnDirectoryListCommand)
}

pub fn commands_factory(file_name_or_directory:&str, command: Commands) ->CommandTypes {
    match command {
        Commands::ONDIRECTORYLISTCOMMAND => {
            CommandTypes::OnDirectoryListCommand(OnDirectoryListCommand::new(file_name_or_directory.to_string()))
        },
        Commands::ONFILEREADCOMMAND => {
            CommandTypes::OnFileReadCommand(OnFileReadCommand::new(file_name_or_directory.to_string()))
        }
    }
}