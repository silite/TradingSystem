#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::sync::{Arc, Mutex};

use derive_builder::Builder;
use market_feed::MarketFeed;
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{command::Command, market::Market};
use strategy::StrategyExt;

/// Clone only for Builder.
#[derive(Builder, Clone)]
pub struct Trader<Portfolio, MarketData, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketData: MarketFeed,
    Strategy: StrategyExt,
{
    /// Used as a unique identifier seed for the Portfolio, Trader & Positions associated with this [`Engine`].
    engine_id: uuid::Uuid,
    ///
    market: Market,
    /// receiving [`Command`]s from a remote source.
    command_rx: crossbeam::channel::Receiver<Command>,
    ///
    portfolio: Portfolio,
    ///
    market_data: Arc<MarketData>,
    ///
    execution: Execution,
    ///
    strategy: Strategy,
}

impl<Portfolio, MarketData, Execution, Strategy> Trader<Portfolio, MarketData, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketData: MarketFeed,
    Strategy: StrategyExt,
{
    pub fn new() -> Self {
        todo!()
    }
}
