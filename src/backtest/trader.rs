use std::sync::Arc;

use execution::{virtual_matching::VirtualMatching, ExecutionExt};
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

use super::MARKET_FEED_COMMAND_TOPIC;

pub async fn init_binance_trader<Portfolio>(
    engine_id: Uuid,
    market: Market,
    portfolio: Portfolio,
    command_bus: Arc<CommandBus>,
) -> Trader<Portfolio, VirtualMatching, MacdStrategy>
where
    Portfolio: BalanceHandler + PositionHandler + Clone,
{
    let execution = VirtualMatching::new();
    let order_resp_rx = execution.get_order_resp_rx();

    let macd_strategy = MacdStrategyBuilder::default()
        .config(MacdStrategyConfig {
            open_interval: 1000,
            adx_threshold: 0.1,
            macd_diff: 0.,
            rsi_diff: 0.,
            atr_scaling: 1.,
            per_hand: 200.,
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
            MARKET_FEED_COMMAND_TOPIC,
        ))
        .command_queue(Default::default())
        .execution(Arc::new(execution))
        .order_resp_rx(order_resp_rx)
        .strategy(macd_strategy)
        .build()
        .expect("init trader error.")
}
