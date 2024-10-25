use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use protocol::{
    event::{bus::EventBus, MarketFeedEvent},
    market::{InstrumentKind, Market},
};

use super::{portfolio::init_portfolio, trader::init_binance_trader};

pub async fn start_engine() -> anyhow::Result<Arc<EventBus>> {
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
            init_binance_trader(engine_id, market, portfolio, event_bus.clone()).await,
        )]))
        .build()?
        .run()?;

    Ok(event_bus)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_start_engine() {
    utils::logs::logs_guard();
    let event_bus = start_engine().await.unwrap();
    sleep(Duration::from_secs(5));
    event_bus
        .publish(
            "binance_market_feed",
            protocol::event::Event::MarketFeed(MarketFeedEvent::LoadHistory),
        )
        .unwrap();
    loop {}
}
