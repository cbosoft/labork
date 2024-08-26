mod action;
mod actor;
mod config;
mod error;
mod logging;
mod ork;

use crate::config::Config;
use crate::ork::Ork;
use crate::logging::setup_logger;



#[tokio::main]
async fn main() {
    setup_logger().unwrap();
    let config = Config::open().unwrap();

    let ip = "127.0.0.1";
    let port = 3000;
    let addr = format!("{ip}:{port}");

    let app = axum::Router::new();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let ork = Ork::open().unwrap();
    tokio::task::spawn(ork.run());
    log::info!("Listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
