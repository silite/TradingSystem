use std::collections::HashMap;

use protocol::market::{InstrumentKind, Market};

use super::{portfolio::init_portfolio, trader::init_binance_trader};

pub async fn start_engine() -> anyhow::Result<()> {
    let (_command_tx, command_rx) = crossbeam::channel::unbounded();
    let engine_id = uuid::Uuid::new_v4();
    let market = Market::new("binance", ("btc", "usdt", InstrumentKind::Future));
    let portfolio = init_portfolio(engine_id, market.clone());

    engine::EngineBuilder::default()
        .engine_id(engine_id)
        .command_rx(command_rx)
        .config(conf::CONFIG.clone())
        .portfolio(HashMap::from([(market.clone(), portfolio.clone())]))
        .traders(HashMap::from([(
            market.clone(),
            init_binance_trader(engine_id, market, portfolio).await,
        )]))
        .build()?
        .run()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_start_engine() {
    start_engine().await.unwrap();
    loop {}
}
