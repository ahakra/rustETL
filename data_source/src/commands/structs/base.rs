use crate::commands::enums::Commands;
#[derive(Debug)]

pub struct Base {
    pub dir_or_file :String,
    pub command: Commands
}