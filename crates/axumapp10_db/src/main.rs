use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=postgres dbname=axum password=postgres",
        NoTls,
    )?;
    let pool = Pool::builder().build(manager).await?;
    let app = Router::new()
        .route("/query_from_db", get(query_from_db))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404)
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}

async fn query_from_db(State(pool): State<ConnectionPool>) -> Result<String, (StatusCode, String)> {
    tracing::debug!("get db conn {:?}", pool);
    let conn = pool.get().await.map_err(internal_error)?;

    tracing::debug!("query_from_db: 1");
    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    tracing::debug!("query_from_db: 2");

    let two: i32 = row.try_get(0).map_err(internal_error)?;
    tracing::debug!("query_from_db: 3");
    tracing::debug!("calc_result {:?}", two);

    Ok(two.to_string())
}
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
