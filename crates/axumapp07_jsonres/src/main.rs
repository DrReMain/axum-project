use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/resjson", post(res_json))
        .route("/resjson2", post(res_json2))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Input {
    name: String,
    email: String,
}
async fn res_json(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
    Json(json!({
        "result": "ok",
        "number": 1
    }))
}

#[derive(Debug, Serialize)]
struct Output {
    name: String,
    age: u32,
}
async fn res_json2(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
    let a = Output {
        name: "mike".to_string(),
        age: 20,
    };
    Json(serde_json::to_value(a).unwrap())
    // Redirect::to("/")
    // (StatusCode::OK, "hello world")
}
