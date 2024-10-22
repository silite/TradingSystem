use std::{sync::Arc, thread::JoinHandle};

use derive_builder::Builder;
use protocol::event::MarketEvent;

use crate::StrategyExt;

#[derive(Builder, Clone)]
pub struct MacdStrategy {
    event_rx: crossbeam::channel::Receiver<MarketEvent>,
}

impl StrategyExt for MacdStrategy {
    fn handle_event(
        self,
        rx: crossbeam::channel::Receiver<MarketEvent>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let this = Arc::new(self);
        std::thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                match event.kind {
                    protocol::event::DataKind::Kline(kline) => todo!(),
                    protocol::event::DataKind::BundleData(bundle_market_indicator) => {
                        this.pre_valid()?;
                        this.try_close()?;
                        this.try_open()?;
                        this.rear_valid()?;
                    }
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
