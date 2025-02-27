use axum::{extract::Query, response::Html, routing::get, Form, Router};
use serde::Deserialize;
use tower_http::trace::TraceLayer;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .route("/form", get(show_form).post(accept_form))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    tracing::debug!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn query(Query(params): Query<Params>) -> Html<&'static str> {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
            </head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name" />
                    </label>

                    <label for="email">
                        Enter your email:
                        <input type="text" name="email" />
                    </label>

                    <input type="submit" value="Subscribe!" />
                </form>
            </body>
        </html>
        "#,
    )
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Input {
    name: String,
    email: String,
}
async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form params {:?}", input);
    Html("<h3>Form posted</h3>")
}
