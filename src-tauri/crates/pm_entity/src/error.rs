use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("db error: {0}")]
    DbError(String),
}
