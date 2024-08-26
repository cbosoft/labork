use std::net::Ipv4Addr;

use crate::ork::Ork;


pub async fn run_api(ip: Ipv4Addr, port: u16,ork: Ork) {
    let app = axum::Router::new()
        .with_state(ork);
    let listener = tokio::net::TcpListener::bind((ip, port)).await.unwrap();

    log::info!("API Listening on {ip:?}:{port}");
    axum::serve(listener, app)
        .await
        .unwrap();
}
