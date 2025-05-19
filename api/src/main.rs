use api::{db, router::build_router};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let db: PgPool = db::init_db()
        .await
        .expect("failed to connect to the database!");
    let router = build_router(db.clone());

    println!("serving api on localhost:3000");

    let listener = TcpListener::bind("localhost:3000")
        .await?;
    axum::serve(listener, router).await?;

    Ok(())
}