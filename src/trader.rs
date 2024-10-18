use market_feed::{data::binance::BinanceMarketFeed, MarketFeed};
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::market::Market;
use strategy::{
    implements::macd::{MacdStrategy, MacdStrategyBuilder},
    StrategyExt,
};
use trader::Trader;
use uuid::Uuid;

pub fn init_binance_trader<Portfolio>(
    engine_id: Uuid,
    market: Market,
    portfolio: Portfolio,
) -> Trader<Portfolio, BinanceMarketFeed, (), MacdStrategy>
where
    Portfolio: BalanceHandler + PositionHandler + Clone,
{
    let (_command_tx, command_rx) = crossbeam::channel::unbounded();

    trader::TraderBuilder::default()
        .engine_id(engine_id)
        .market(market)
        .command_rx(command_rx)
        .portfolio(portfolio)
        .market_data(BinanceMarketFeed::new())
        .execution(())
        .strategy(
            MacdStrategyBuilder::default()
                .build()
                .expect("init macd ver strategy error."),
        )
        .build()
        .expect("init trader error.")
}
