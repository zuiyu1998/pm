use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::DbConfig;

pub const APP_ORGANIZATION: &str = "next";
pub const APP_NAME: &str = "pm";

pub struct Config {
    proj_dirs: ProjectDirs,
}

impl Config {
    pub fn new() -> Self {
        let proj_dirs =
            ProjectDirs::from("com", APP_ORGANIZATION, APP_NAME).expect("Os not support.");
        Self { proj_dirs }
    }

    pub fn get_db_config(&self) -> DbConfig {
        let sqlite = self.sqlite();
        let sqlite = sqlite.to_string_lossy();

        DbConfig {
            database_url: format!("sqlite://{sqlite}"),
        }
    }

    pub fn data(&self) -> &Path {
        self.proj_dirs.data_dir()
    }

    pub fn sqlite(&self) -> PathBuf {
        self.proj_dirs.data_dir().join("sqlite.db")
    }
}
