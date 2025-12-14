use crate::Error;
use migration::{Migrator, MigratorTrait};
use pm_entity::TaskRepo;
use pm_sea_orm::task::SeaOrmTaskRepo;

pub struct DbConfig {
    pub database_url: String,
}

pub struct DbRepo {
    pub task: Box<dyn TaskRepo>,
}

impl DbRepo {
    pub async fn init_db(config: &DbConfig) -> Result<DbRepo, Error> {
        let connection = sea_orm::Database::connect(&config.database_url).await?;
        Migrator::up(&connection, None).await?;

        let task = SeaOrmTaskRepo::new(connection);

        Ok(DbRepo {
            task: Box::new(task),
        })
    }
}
