use std::net::Ipv4Addr;

use tower_http::services::ServeDir;

use crate::ork::Ork;


pub async fn run_ui(ip: Ipv4Addr, port: u16, ork: Ork) {
    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("public"))
        .with_state(ork);
    let listener = tokio::net::TcpListener::bind((ip, port)).await.unwrap();

    log::info!("UI serving from http://{ip:?}:{port}");
    axum::serve(listener, app)
        .await
        .unwrap();
}

