
#[derive(Debug)]
pub struct OnFileReadCommand {
    pub file_name:String

}

impl OnFileReadCommand {
    pub fn new(file_name: String) -> Self {

            return OnFileReadCommand { file_name }
        }
    }
