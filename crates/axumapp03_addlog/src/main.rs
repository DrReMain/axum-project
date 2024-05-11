use axum::{response::Html, routing::get, Router};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
    let app = Router::new()
        .route("/foo", get(handler))
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    tracing::debug!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}
