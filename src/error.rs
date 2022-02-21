use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    /// ipfs is not found
    UndefinedError,
    FileNotUploaded,
    FileNotFound,
    ContractNotDeployed,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::UndefinedError => write!(f, "Undefined error occurred"),
            ErrorKind::FileNotUploaded => write!(f, "File could not be uploaded, check if ipfs is running"),
            ErrorKind::FileNotFound => write!(f, "File not found"),
            ErrorKind::ContractNotDeployed => write!(f, "Contract could not be deployed"),
        }
    }
}
