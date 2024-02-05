use crate::commands::enums::Commands;
use crate::commands::structs::base::Base;

#[derive(Debug)]
pub struct OnDirectoryListCommand {
    pub super_struct:Base,


}


impl OnDirectoryListCommand {
    pub fn new(directory_path: String) -> Self {

        let base =Base{
            dir_or_file : directory_path.to_string(),
            command: Commands::ONDIRECTORYLISTCOMMAND
        };
     return OnDirectoryListCommand{ super_struct:base}
    }
}
