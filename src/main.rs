use axum::{Router, routing::get};

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handlers::root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
