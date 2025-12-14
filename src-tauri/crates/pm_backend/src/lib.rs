mod config;
mod db;

pub mod error;

use std::sync::Arc;

pub use config::*;
pub use db::*;
pub use error::Error;

use tokio::fs::{self, File};

pub struct AppState {
    pub db: Arc<DbRepo>,
}

impl AppState {
    pub async fn init_app_state(config: &Config) -> Result<AppState, Error> {
        let data_dir = config.data();

        if !data_dir.exists() {
            fs::create_dir_all(data_dir).await?;
        }

        let sqlite = config.sqlite();

        if !sqlite.exists() {
            File::create(sqlite).await?;
        }

        let db = DbRepo::init_db(&config.get_db_config()).await?;

        Ok(AppState { db: Arc::new(db) })
    }
}
