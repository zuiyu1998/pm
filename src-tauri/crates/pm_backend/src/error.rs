use sea_orm::DbErr;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    IoError(#[from] IoError),
}
