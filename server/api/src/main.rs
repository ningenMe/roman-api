mod controller;

use generated::server::MakeService;
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;

#[tokio::main]
async fn main() {
    let server = controller::bookmark_controller::Server::new();
    #[allow(unused_mut)]
    let mut service =
        generated::server::context::MakeAddContext::<_, EmptyContext>::new(
            MakeAllowAllAuthenticator::new(MakeService::new(server), "cosmo")
        );

    let addr = "0.0.0.0:50051".parse().expect("Failed to parse bind address");
    hyper::server::Server::bind(&addr).serve(service).await.unwrap()
}
