use std::env;
use std::error::Error;

use sea_orm::{Database, DatabaseConnection};

pub async fn get_connection() -> Result<DatabaseConnection, Box<dyn Error>> {
    let username = env::var("USERNAME")?;
    let password = env::var("PASSWORD")?;
    let host = env::var("HOST")?;
    let port = env::var("PORT")?;
    let dbname = env::var("DBNAME")?;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, dbname
    );

    let connection = Database::connect(connection_string).await?;

    Ok(connection)
}
