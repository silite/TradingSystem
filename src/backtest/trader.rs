use std::sync::Arc;

use market_feed::{data::binance::BinanceMarketFeed, MarketFeed};
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{
    event::bus::CommandBus, indictor::Indicators, market::Market,
    portfolio::market_data::binance::Kline,
};
use strategy::{
    implements::macd::{MacdStrategy, MacdStrategyBuilder, MacdStrategyConfig},
    StrategyExt,
};
use trader::Trader;
use uuid::Uuid;

pub async fn init_binance_trader<Portfolio>(
    engine_id: Uuid,
    market: Market,
    portfolio: Portfolio,
    command_bus: Arc<CommandBus>,
) -> Trader<Portfolio, (), MacdStrategy>
where
    Portfolio: BalanceHandler + PositionHandler + Clone,
{
    let indicator_market_feed_topic = "indicator_strategy";
    let market_feed_command_topic = "binance_market_feed";

    let macd_strategy = MacdStrategyBuilder::default()
        .market_feed_topic(indicator_market_feed_topic)
        .config(MacdStrategyConfig {
            open_interval: 1000,
            adx_threshold: 0.1,
            macd_diff: 0.,
            rsi_diff: 0.,
        })
        .state(Default::default())
        .indicators(None)
        .market_data(None)
        .build()
        .expect("Init macd strategy error.");

    trader::TraderBuilder::default()
        .engine_id(engine_id)
        .market(market)
        .command_bus(command_bus.clone())
        .portfolio(portfolio)
        .market_feed_rx(BinanceMarketFeed::new(
            command_bus.clone(),
            market_feed_command_topic,
        ))
        .command_queue(Default::default())
        .execution(())
        .strategy(macd_strategy)
        .build()
        .expect("init trader error.")
}
