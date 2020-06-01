use sqlx::postgres::PgPool;
use std::env;

pub async fn establish_connection() -> Result<PgPool, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;
    Ok(PgPool::builder().build(&db_url).await?)
}
