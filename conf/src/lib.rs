use std::sync::LazyLock;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(long_about = None, ignore_errors = true, name="TradingSystem", author="rongjiale", version="0.0.1")]
struct Cli {
    #[arg(
        short = 'c',
        long = "config",
        default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/dev.toml"),
        help = "Path to configuration file"
    )]
    config: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub clickhouse: String,
    pub db_name: String,
}

fn init_config() -> anyhow::Result<Config> {
    let load_settings = || {
        let cli: Cli = Cli::parse();
        let mut settings = config::Config::builder();
        settings = settings.add_source(config::File::with_name(&cli.config));
        settings
    };
    let settings = load_settings();
    let web_config: Config = settings.build()?.try_deserialize()?;
    Ok(web_config)
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| init_config().unwrap());
