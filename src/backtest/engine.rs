use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use protocol::{
    event::{bus::CommandBus, MarketFeedCommand},
    market::{InstrumentKind, Market},
};

use super::{portfolio::init_portfolio, trader::init_binance_trader};

pub async fn start_engine() -> anyhow::Result<Arc<CommandBus>> {
    let engine_id = uuid::Uuid::new_v4();
    let market = Market::new("binance", ("btc", "usdt", InstrumentKind::Future));
    let portfolio = init_portfolio(engine_id, market.clone());
    let command_bus = Arc::new(CommandBus::new());

    engine::EngineBuilder::default()
        .engine_id(engine_id)
        .command_bus(command_bus.clone())
        .config(conf::CONFIG.clone())
        .portfolio(HashMap::from([(market.clone(), portfolio.clone())]))
        .traders(HashMap::from([(
            market.clone(),
            init_binance_trader(engine_id, market, portfolio, command_bus.clone()).await,
        )]))
        .build()?
        .run()?;

    Ok(command_bus)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_start_engine() {
    utils::logs::logs_guard();
    let command_bus = start_engine().await.unwrap();
    sleep(Duration::from_secs(5));
    command_bus
        .publish(
            "binance_market_feed",
            protocol::event::Command::MarketFeed(MarketFeedCommand::LoadHistory),
        )
        .unwrap();
    loop {}
}
