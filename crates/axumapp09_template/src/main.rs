use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/greet/:name", get(greet))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    Html(HelloTemplate { name }.to_string())
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "noting to see here")
}
