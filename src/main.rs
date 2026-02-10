use std::{
    env,
    io::{Error, Result},
    num::ParseIntError,
};

use axum::response::Html;
use axum::routing::get;
use axum::{http::StatusCode, Router};

async fn index(body: String) -> Html<String> {
    Html(format!("<p>Hello, {}!</p>", body))
}

async fn ok() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let port = match env::var("PORT") {
        Ok(v) => v
            .parse()
            .map_err(|e: ParseIntError| Error::other(e.to_string()))?,
        Err(_) => 8080,
    };

    let app = Router::new()
        .route("/", get(index).post(index))
        .route("/health/readiness", get(ok))
        .route("/health/liveness", get(ok));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
