use tracing::info;
use tracing_subscriber;
use thiery_lib;
mod config;
use config::Config;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    let subscriber = tracing_subscriber::fmt::fmt()
       .with_level(true)
       .with_target(true)
       .pretty();

    tracing::subscriber::set_global_default(subscriber.finish())
        .expect("setting default subscriber failed");

    info!("ThieryLib version used : {}", thiery_lib::version());

    // Initialize and load configuration
    Config::init().await.unwrap();
    let cfg = Config::load();
    info!("Config loaded: {:?}", cfg);
}