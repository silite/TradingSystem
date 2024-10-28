use std::time::Duration;

use protocol::event::MarketFeedCommand;
use utils::logs;

mod engine;
mod portfolio;
mod trader;
pub const MARKET_FEED_COMMAND_TOPIC: &'static str = "binance_market_feed";

pub async fn start() -> anyhow::Result<()> {
    //
    logs::logs_guard();

    //
    let command_bus = engine::start_engine().await?;

    std::thread::sleep(Duration::from_secs(5));
    command_bus
        .publish(
            MARKET_FEED_COMMAND_TOPIC,
            protocol::event::Command::MarketFeed(MarketFeedCommand::LoadHistory),
        )
        .unwrap();

    loop {}
}
