mod config;
mod engine;
mod linux;
mod service;

use config::Config;
use engine::SelectionEngine;
use linux::LinuxAccessibility;
use service::SmartSelectService;

#[tokio::main]
async fn main() -> Result<(), String> {
    let config = Config::default();

    let engine =
        SelectionEngine::new(config);

    let accessibility =
        LinuxAccessibility::connect().await?;

    let mut service =
        SmartSelectService::new(
            engine,
            accessibility,
        );

    service.run().await
}