use std::io;

#[derive(Debug)]
pub enum SftpError {
    ConnectionError,
    ListingError,
    FieldMappingError,
    SftpConfigError,

}

pub enum JSONError {
    ParsingError(io::Error),
    CannotFindFileError(io::Error),
}

impl std::fmt::Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSONError::ParsingError(err) => write!(f, "Cannot parse file ,{}",err),
            JSONError::CannotFindFileError(err) => write!(f, "File not found ,{}",err),
        }
    }
}