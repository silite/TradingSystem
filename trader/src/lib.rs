#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use market_feed::MarketFeed;
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{
    event::{bus::EventBus, Event},
    market::Market,
};
use strategy::StrategyExt;

/// Clone only for Builder.
#[derive(Builder, Clone)]
pub struct Trader<Portfolio, MarketFeeder, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketFeeder: MarketFeed,
    Strategy: StrategyExt,
    Execution: Send,
{
    /// Used as a unique identifier seed for the Portfolio, Trader & Positions associated with this [`Engine`].
    engine_id: uuid::Uuid,
    ///
    market: Market,
    /// receiving [`Command`]s from a remote source.
    event_bus: Arc<EventBus>,
    ///
    portfolio: Portfolio,
    ///
    market_feed: MarketFeeder,
    ///
    execution: Execution,
    ///
    strategy: Strategy,
}

impl<Portfolio, MarketFeeder, Execution, Strategy>
    Trader<Portfolio, MarketFeeder, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketFeeder: MarketFeed + Send + 'static,
    Strategy: StrategyExt + Send + 'static,
    Execution: Send,
{
    pub fn new() -> Self {
        todo!()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        ftlog::info!("[trade] {} {:?} run.", self.engine_id, self.market);

        let event_bus_cp = self.event_bus.clone();
        // 等待spawn成功
        let (start_tx, start_rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let _ = start_tx.send(());
            self.strategy.run(event_bus_cp.clone());
            let _ = self.market_feed.run().await.map_err(|err| {
                ftlog::error!("[market feed] run error. {:?}", err);
            });
        });
        start_rx.await?;

        let event_rx = self
            .event_bus
            .subscribe(format!("{:?}-trader", self.market));
        'trading: loop {
            if let Ok(command) = event_rx.recv() {
                match command {
                    Event::Command(command_event) => match command_event {
                        protocol::event::CommandEvent::Terminate(_) => break 'trading,
                    },
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }
}
