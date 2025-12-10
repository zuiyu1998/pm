mod task;

pub mod error;

pub use task::*;
pub use error::Error;

pub struct DbRepo {
    pub db: Box<dyn TaskRepo>,
}
