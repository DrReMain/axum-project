use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}
