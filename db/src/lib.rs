pub mod market_data;
use std::sync::LazyLock;

use clickhouse::Client;
use conf::CONFIG;

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let client = Client::default();
    // client
    //     .with_url(&CONFIG.clickhouse)
    //     .with_user("default")
    //     .with_database(&CONFIG.db_name)
    client
        .with_url("http://127.0.0.1:8123")
        .with_user("default")
        .with_database("crypto")
});
