use refinery::config::{Config, ConfigDbType};
use std::env;

mod migrations;

// Migration is run synchronously since sqlx is not natively
// supported by Refinery.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_host = env::var("DB_HOST")?;
    let db_port = env::var("DB_PORT")?;
    let db_user = env::var("DB_USER")?;
    let db_pass = env::var("DB_PASS")?;
    let db_name = env::var("DB_NAME")?;

    // Refinery and sqlx seem to be using configuration key
    // differently. Thus we need to set it like so
    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_user(&db_user)
        .set_db_pass(&db_pass)
        .set_db_host(&db_host)
        .set_db_port(&db_port)
        .set_db_name(&db_name);
    println!("Running migrations");
    migrations::runner().run(&mut conn)?;
    Ok(())
}
