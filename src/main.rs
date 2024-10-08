mod api;
mod action;
mod actor;
mod config;
mod error;
mod logging;
mod ork;
mod ui;

use crate::config::Config;
use crate::ork::Ork;
use crate::logging::setup_logger;
use crate::api::run_api;
use crate::ui::run_ui;



#[tokio::main]
async fn main() {
    setup_logger().unwrap();
    let config = Config::open().unwrap();

    let ork = Ork::open().unwrap();

    let api_task = run_api(config.api_ip, config.api_port, ork.clone());
    let ui_task = run_ui(config.ui_ip, config.ui_port, ork.clone());
    let main_task = ork.run();
    let _ = tokio::join!(api_task, ui_task, main_task);
}
