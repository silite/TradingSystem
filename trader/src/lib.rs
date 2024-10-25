#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod machine;

use std::{
    collections::VecDeque,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use market_feed::{indictor, MarketFeed};
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{
    event::{bus::CommandBus, TradeEvent},
    indictor::Indicators,
    market::Market,
};
use strategy::StrategyExt;
use yata::core::OHLCV;

/// Clone only for Builder.
#[derive(Builder, Clone)]
pub struct Trader<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    Strategy: StrategyExt,
    Execution: Send,
{
    /// Used as a unique identifier seed for the Portfolio, Trader & Positions associated with this [`Engine`].
    engine_id: uuid::Uuid,
    /// 交易所标的
    market: Market,
    /// receiving [`Command`]s from a remote source.
    command_bus: Arc<CommandBus>,
    /// 资券
    portfolio: Portfolio,
    /// 行情、指标源
    market_feed_rx:
        crossbeam::channel::Receiver<(<Strategy as StrategyExt>::MarketData, Indicators)>,
    /// 执行器
    execution: Execution,
    /// 策略
    strategy: Strategy,
    /// 事件循环队列
    command_queue: VecDeque<TradeEvent<<Strategy as StrategyExt>::MarketData>>,
}

impl<Portfolio, Execution, Strategy> Trader<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler + Send + 'static,
    Strategy: StrategyExt + Send + 'static,
    Execution: Send + 'static,
{
    /// trader.run时，策略也要run，监听事件。market_feed.run晚于strategy.run。
    pub async fn run(self) -> anyhow::Result<()> {
        ftlog::info!("[trade] {} {:?} run.", self.engine_id, self.market);
        self.event_loop()
    }

    /// 事件循环
    pub fn event_loop(mut self) -> anyhow::Result<()> {
        loop {
            if let Ok(market) = self.market_feed_rx.recv() {
                self.command_queue.push_back(TradeEvent::Market(market));
            } else {
            }

            while let Some(event) = self.command_queue.pop_front() {
                match event {
                    TradeEvent::Market((market_data, indicators)) => {
                        self.strategy.handle_data(market_data, indicators);
                    }
                }
            }
        }
    }
}
