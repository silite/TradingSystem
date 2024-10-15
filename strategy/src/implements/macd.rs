use derive_builder::Builder;

use crate::StrategyExt;

#[derive(Builder, Clone)]
pub struct MacdStrategy {}

impl StrategyExt for MacdStrategy {
    fn handle_signal(signal: impl crate::signal::SignalExt) {
        todo!()
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
