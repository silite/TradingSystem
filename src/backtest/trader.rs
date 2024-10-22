use std::sync::Arc;

use market_feed::{
    data::binance::BinanceMarketFeed,
    generator::{binance::BinanceMarketGenerator, MarketGenerator},
    MarketFeed,
};
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{
    event::MarketEvent, indictor::BundleMarketIndicator, market::Market,
    portfolio::market_data::binance::Kline,
};
use strategy::{
    implements::macd::{MacdStrategy, MacdStrategyBuilder},
    StrategyExt,
};
use trader::Trader;
use uuid::Uuid;

pub async fn init_binance_trader<Portfolio>(
    engine_id: Uuid,
    market: Market,
    portfolio: Portfolio,
) -> Trader<Portfolio, BinanceMarketGenerator<MarketEvent>, (), MacdStrategy>
where
    Portfolio: BalanceHandler + PositionHandler + Clone,
{
    let (_command_tx, command_rx) = crossbeam::channel::unbounded();
    let (event_tx, event_rx) = crossbeam::channel::unbounded();
    let (binance_market_feed, market_command_tx) = BinanceMarketFeed::new();

    let macd_strategy = MacdStrategyBuilder::default()
        .event_rx(event_rx)
        .build()
        .expect("init macd ver strategy error.");

    trader::TraderBuilder::default()
        .engine_id(engine_id)
        .market(market)
        .event_tx(event_tx)
        .command_rx(command_rx)
        .portfolio(portfolio)
        .market_data_generator(BinanceMarketGenerator::new(market_command_tx))
        .execution(())
        .strategy(macd_strategy)
        .build()
        .expect("init trader error.")
}
