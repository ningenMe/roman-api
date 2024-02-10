mod controller;

#[tokio::main]
async fn main() {
    let api = controller::bookmark_controller::Api{};
    let router = generated::server::new(api);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:50051")
        .await
        .unwrap();
    axum::serve(listener, router)
    .await
    .unwrap();
}
