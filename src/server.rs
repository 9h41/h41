use axum::{Router, response::Html, routing::{get, post}, extract::Path};
use axum::http::{StatusCode, HeaderMap};
use std::net::SocketAddr;

use crate::ports;

const INDEX_HTML: &str = include_str!("../assets/index.html");

pub async fn start(port: u16) {
    let app = Router::new()
        .route("/", get(home))
        .route("/scan", get(scan))
        .route("/kill/{pid}", post(kill_process));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("👂 Started ports web server at http://localhost:{port}, CTRL+C to exit...");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}

async fn home() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn scan() -> String {
    tokio::task::spawn_blocking(ports::all)
        .await
        .map(|entries| serde_json::to_string(&entries).unwrap_or_default())
        .unwrap_or_default()
}

async fn kill_process(headers: HeaderMap, Path(pid): Path<i64>) -> StatusCode {
    // Require X-Requested-With header to prevent CSRF via simple form submissions
    if !headers.contains_key("x-requested-with") {
        return StatusCode::FORBIDDEN;
    }

    if pid <= 0 {
        return StatusCode::BAD_REQUEST;
    }

    let result = tokio::task::spawn_blocking(move || {
        use std::process::Command;
        Command::new("kill").arg(pid.to_string()).status()
    })
    .await;

    match result {
        Ok(Ok(status)) if status.success() => StatusCode::OK,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
