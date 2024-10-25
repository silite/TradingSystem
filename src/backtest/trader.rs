use std::sync::Arc;

use market_feed::{data::binance::BinanceMarketFeed, MarketFeed};
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{
    event::EventBus, indictor::Indicators, market::Market, portfolio::market_data::binance::Kline,
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
    event_bus: Arc<EventBus>,
) -> Trader<Portfolio, BinanceMarketFeed, (), MacdStrategy>
where
    Portfolio: BalanceHandler + PositionHandler + Clone,
{
    let indicator_market_feed_topic = "indicator_strategy";
    let market_feed_event_topic = "binance_market_feed";

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
        .event_bus(event_bus.clone())
        .portfolio(portfolio)
        .market_feed(BinanceMarketFeed::new(
            event_bus.clone(),
            indicator_market_feed_topic,
            market_feed_event_topic,
        ))
        .execution(())
        .strategy(macd_strategy)
        .build()
        .expect("init trader error.")
}
