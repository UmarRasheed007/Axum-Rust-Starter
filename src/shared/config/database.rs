use crate::CONFIG;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr};

pub struct Database {
    connection: DatabaseConnection,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init() -> Result<Self, DbErr>
    where
        Self: Sized;
    fn get_connection(&self) -> &DatabaseConnection;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn init() -> Result<Self, DbErr> {
        // Get the database URL from the parameters
        let database_url = &CONFIG.database.database_url;

        // Connect to PostgreSQL using SeaORM
        let connection = sea_orm::Database::connect(database_url).await?;

        Ok(Self { connection })
    }

    fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
