mod task;

pub use task::*;

pub struct DbRepo {
    pub db: Box<dyn TaskRepo>,
}
