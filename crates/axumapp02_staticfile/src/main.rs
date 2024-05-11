use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serve_dir = ServeDir::new("assets").not_found_service(ServeDir::new("assets/index.html"));

    let app = Router::new()
        .route("/foo", get(handler))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
async fn handler() -> Html<&'static str> {
    Html("<h1>From Axum</h1>")
}
