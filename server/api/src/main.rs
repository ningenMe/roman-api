use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

mod controller;

#[tokio::main]
async fn main() {
    let api = controller::bookmark_controller::Api{};
    let layer = CorsLayer::new()
    .allow_origin([
        "https://ningenme.net".parse::<HeaderValue>().unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap()
    ])
    .allow_methods([
        Method::GET, 
        Method::POST,
        Method::OPTIONS
    ])
    ;
    let router = generated::server::new(api).layer(layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:50051")
        .await
        .unwrap();
    axum::serve(listener, router)
    .await
    .unwrap();
}
