use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/", get(|| async { 
            "Hello, World!"
        }));

    println!("serving api on localhost:3000");

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await?;
    axum::serve(listener, app).await?;

    Ok(())
}