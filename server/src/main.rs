//! Run with
//!
//! ```not_rust
//! cargo run -p tokyorust-server
//! ```

use axum::{
    http::{self, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /submission` goes to `code`
        .route("/submission", post(submission))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 9090));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn submission(
    Json(payload): Json<Submission>,
) -> impl IntoResponse {
    println!("{}", payload.code);

    let code = Code {
        code: payload.code,
    };

    (StatusCode::CREATED, Json(code))
}

#[derive(Deserialize)]
struct Submission {
    code: String,
}

#[derive(Serialize)]
struct Code {
    code: String,
}