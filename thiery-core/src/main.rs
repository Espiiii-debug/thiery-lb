use color_eyre::eyre::Result;
use thiery_lib;

mod config;
use config::Config;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Thiery-LB {}", thiery_lib::version());

    Config::init()?;
    let mut _cfg = Config::load()?;

    Ok(())
}