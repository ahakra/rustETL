use crate::commands::enums::Commands;
use crate::commands::structs::base::Base;

#[derive(Debug)]
pub struct OnFileReadCommand {
    pub super_struct:Base

}

impl OnFileReadCommand {
    pub fn new(file_name: String) -> Self {

            let base = Base {
                dir_or_file: file_name.to_string(),
                command: Commands::ONDIRECTORYLISTCOMMAND
            };
            return OnFileReadCommand { super_struct: base }
        }
    }
