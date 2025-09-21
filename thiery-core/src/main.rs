use color_eyre::eyre::Result;
use tracing::info;
use tracing_subscriber;
use thiery_lib;
mod config;
use config::Config;

fn main() -> Result<()> {
    color_eyre::install()?;
  
    // Initialize tracing subscriber for logging
    let subscriber = tracing_subscriber::fmt::fmt()
       .with_level(true)
       .with_target(true)
       .pretty();

    tracing::subscriber::set_global_default(subscriber.finish())
        .expect("setting default subscriber failed");

    info!("ThieryLib version used : {}", thiery_lib::version());
  
    Config::init()?;
    let mut _cfg = Config::load()?;
}