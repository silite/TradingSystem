use std::{collections::HashMap, sync::Arc};

use protocol::{
    event::EventBus,
    market::{InstrumentKind, Market},
};

use super::{portfolio::init_portfolio, trader::init_binance_trader};

pub async fn start_engine() -> anyhow::Result<()> {
    let engine_id = uuid::Uuid::new_v4();
    let market = Market::new("binance", ("btc", "usdt", InstrumentKind::Future));
    let portfolio = init_portfolio(engine_id, market.clone());
    let event_bus = Arc::new(EventBus::new());

    engine::EngineBuilder::default()
        .engine_id(engine_id)
        .event_bus(event_bus.clone())
        .config(conf::CONFIG.clone())
        .portfolio(HashMap::from([(market.clone(), portfolio.clone())]))
        .traders(HashMap::from([(
            market.clone(),
            init_binance_trader(engine_id, market, portfolio, event_bus).await,
        )]))
        .build()?
        .run()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn test_start_engine() {
    start_engine().await.unwrap();
    loop {}
}
