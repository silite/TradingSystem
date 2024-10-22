use std::sync::LazyLock;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(long_about = None, ignore_errors = true, name="TradingSystem", author="rongjiale", version="0.0.1")]
struct Cli {
    #[arg(long, required = true)]
    config: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub clickhouse: String,
    pub db_name: String,
}

fn init_config() -> anyhow::Result<Config> {
    #[allow(unused_variables)]
    let load_settings = || {
        let cli: Cli = Cli::parse();
        let mut settings = config::Config::builder();
        settings = settings.add_source(config::File::with_name(&cli.config));
        settings
    };
    #[cfg(feature = "dev")]
    let load_settings = || {
        let mut settings = config::Config::builder();
        settings = settings.add_source(config::File::with_name(
            "/Users/siliterong/Project/rust/TradingSystem/conf/dev.toml",
        ));
        settings
    };
    let settings = load_settings();
    let web_config: Config = settings.build()?.try_deserialize()?;
    Ok(web_config)
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| init_config().unwrap());
