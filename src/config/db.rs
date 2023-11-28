use sea_orm::DatabaseConnection;
use sea_orm::Database;
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DB_POSTGRES")
        .expect("DB_POSTGRES must be set in .env file");

    Database::connect(&database_url)
        .await
        .expect("Database connection failed")
}
