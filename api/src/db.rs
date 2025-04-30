extern crate dotenvy;
use std::env;
use sqlx::PgPool;

pub async fn init_db() -> sqlx::Result<PgPool> {
    dotenvy::dotenv().unwrap();

    let url = env::var("DATABASE_URL")
        .expect("no database url variable set");

    let pool: PgPool = PgPool::connect(&url).await?;

    Ok(pool)
}