use std::env;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, get_service};

use axum_extra::routing::SpaRouter;
use tokio::io;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

const LOG_LEVEL: &str = "warn";
const LISTENING_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", LOG_LEVEL);
    tracing_subscriber::fmt::init();

    let current_dir = env::current_dir().expect("current directory");
    
    let mut dist_dir = current_dir.clone();
    dist_dir.push("dist/");

    let mut assets_dir = current_dir.clone();
    assets_dir.push("assets/");

    let assets_service = get_service(ServeDir::new(assets_dir)).handle_error(handle_error);

    // TODO SpaRouter
    let app = Router::new()
        .route("/api/root", get(root))
        .merge(SpaRouter::new("/", dist_dir))
        .nest_service("/assets/", assets_service)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    tracing::info!("listening on {}", LISTENING_ADDRESS);
    axum::Server::bind(&LISTENING_ADDRESS)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    "Hello, World!"
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}