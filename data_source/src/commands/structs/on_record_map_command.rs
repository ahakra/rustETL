#[derive(Debug)]
pub struct OnRecordMapCommand {
    pub file_path:String,


}


impl OnRecordMapCommand {
    pub fn new(directory_path: String) -> Self {
     
     return OnDirectoryListCommand{ directory_path}
    }
}
