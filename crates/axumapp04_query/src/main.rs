use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: i32,
    bar: String,
    aa: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    tracing::debug!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
async fn query(Query(params): Query<Params>) -> impl IntoResponse {
    tracing::debug!("query params {:?}", params);
    Html(format!("<h3>Test query {:?}</h3>", params))
}
