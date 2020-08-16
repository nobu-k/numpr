use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, instrument};
use tracing_subscriber::fmt;

#[instrument(skip(_req))]
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_target(true)
        .with_thread_ids(true)
        .with_current_span(true)
        .with_timer(fmt::time::ChronoUtc::rfc3339())
        .flatten_event(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3080));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    info!(port = 3080, "starting the server");
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
