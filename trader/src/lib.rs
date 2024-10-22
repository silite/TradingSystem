#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use market_feed::{
    generator::{Feed, MarketGenerator},
    MarketFeed,
};
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{
    event::{Command, MarketEvent},
    market::Market,
};
use strategy::StrategyExt;

/// Clone only for Builder.
#[derive(Builder, Clone)]
pub struct Trader<Portfolio, MarketDataGenerator, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketDataGenerator: MarketGenerator<MarketEvent>,
    Strategy: StrategyExt,
    Execution: Send,
{
    /// Used as a unique identifier seed for the Portfolio, Trader & Positions associated with this [`Engine`].
    engine_id: uuid::Uuid,
    ///
    market: Market,
    /// receiving [`Command`]s from a remote source.
    command_rx: crossbeam::channel::Receiver<Command>,
    ///
    event_tx: crossbeam::channel::Sender<MarketEvent>,
    ///
    portfolio: Portfolio,
    ///
    market_data_generator: MarketDataGenerator,
    ///
    execution: Execution,
    ///
    strategy: Strategy,
}

impl<Portfolio, MarketDataGenerator, Execution, Strategy>
    Trader<Portfolio, MarketDataGenerator, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketDataGenerator: MarketGenerator<MarketEvent>,
    Strategy: StrategyExt,
    Execution: Send,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn run(mut self) {
        'trading: loop {
            if let Ok(command) = self.command_rx.recv() {
                match command {
                    Command::Terminate(_) => break 'trading,
                }
            }

            match self.market_data_generator.next() {
                Feed::Next(market) => {
                    println!("{:?}", market);
                }
                _ => {
                    ftlog::error!(
                        "[recv market data generator] no handler. {}",
                        self.engine_id
                    );
                }
            }
        }
    }
}
