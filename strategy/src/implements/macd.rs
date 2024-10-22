use std::{sync::Arc, thread::JoinHandle};

use derive_builder::Builder;
use protocol::event::EventBus;

use crate::StrategyExt;

#[derive(Builder, Clone, Default)]
pub struct MacdStrategy {
    market_feed_topic: &'static str,
}

impl StrategyExt for MacdStrategy {
    fn handle_event(self, event_bus: Arc<EventBus>) -> JoinHandle<anyhow::Result<()>> {
        let event_rs = event_bus.subscribe(self.market_feed_topic.to_owned());
        let this = Arc::new(self);
        std::thread::spawn(move || {
            while let Ok(event) = event_rs.recv() {
                match event {
                    protocol::event::Event::MarketData(market_data_event) => {
                        match market_data_event.kind {
                            protocol::event::DataKind::Kline(kline) => todo!(),
                            protocol::event::DataKind::BundleData(bundle_market_indicator) => {
                                this.pre_valid()?;
                                this.try_close()?;
                                this.try_open()?;
                                this.rear_valid()?;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Ok(())
        })
    }

    fn pre_valid(&self) -> anyhow::Result<(), crate::error::PreValidError> {
        todo!()
    }

    fn rear_valid(&self) -> anyhow::Result<(), crate::error::RearValidError> {
        todo!()
    }

    fn try_open(&self) -> anyhow::Result<(), crate::error::OpenError> {
        todo!()
    }

    fn try_close(&self) -> anyhow::Result<(), crate::error::CloseError> {
        todo!()
    }
}
