pub mod market_data;
use std::sync::LazyLock;

use clickhouse::Client;
use conf::CONFIG;

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::default()
        .with_url(&CONFIG.clickhouse)
        .with_user("default")
        .with_database(&CONFIG.db_name)
});
