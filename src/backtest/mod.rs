use std::time::Duration;

use protocol::event::MarketFeedEvent;
use utils::logs;

mod engine;
mod matching;
mod portfolio;
mod trader;

pub async fn start() -> anyhow::Result<()> {
    //
    logs::logs_guard();

    //
    let event_bus = engine::start_engine().await?;

    std::thread::sleep(Duration::from_secs(5));
    event_bus
        .publish(
            "binance_market_feed",
            protocol::event::Event::MarketFeed(MarketFeedEvent::LoadHistory),
        )
        .unwrap();

    loop {}
}
