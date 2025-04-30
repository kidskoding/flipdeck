extern crate tokio;
use api::{db, router::build_router};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db: PgPool = db::init_db()
        .await
        .expect("failed to connect to the database!");
    let router = build_router(db.clone());

    println!("serving api on localhost:3000");

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await?;
    axum::serve(listener, router).await?;

    Ok(())
}