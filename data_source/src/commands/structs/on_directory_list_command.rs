

#[derive(Debug)]
pub struct OnDirectoryListCommand {
    pub directory_path:String,


}


impl OnDirectoryListCommand {
    pub fn new(directory_path: String) -> Self {
     
     return OnDirectoryListCommand{ directory_path}
    }
}
