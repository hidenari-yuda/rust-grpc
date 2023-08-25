pub mod label;
pub mod todo;

use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(i32),
    #[error("Duplicated Error: [{0}]")]
    Duplicate(i32),
}
