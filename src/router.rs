use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::{Body, Server, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use controllers::file_controller::FileController;

pub async fn start_router(port: u16, db_url: String) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = TcpListener::bind(addr).await.expect("failed to bind");

    let new_service = make_service_fn(move |_| {
        let controller = FileController::new(db_url.clone());
        async move { Ok::<_, hyper::Error>(service_fn(move |req| {
            controller.handle_request(req)
        })) }
    });

    let server = Server::from_tcp(listener).unwrap();

    server.serve(new_service).await.unwrap();
}