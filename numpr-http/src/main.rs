use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, instrument};
use tracing_subscriber::fmt;

#[instrument(skip(req))]
async fn hello_world(req: Request<Body>) -> http::Result<Response<Body>> {
    let uri = req.uri();
    match uri.path() {
        "/problems" if req.method() == hyper::Method::GET => {
            Ok(Response::new("Hello, World".into()))
        }
        "/problems" if req.method() == hyper::Method::POST => Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body("Not implemented yet".into()),
        _ => Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body("Not Found".into()),
    }
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
