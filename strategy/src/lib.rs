#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::sync::Arc;

use error::{CloseError, OpenError, PreValidError, RearValidError};
use protocol::{event::bus::CommandBus, indictor::Indicators, order::OrderRequest, trade::Side};
use yata::{core::OHLCV, indicators};

mod error;
pub mod implements;

pub trait StrategyExt {
    type MarketData: OHLCV + Send + Clone;
    type Config;

    fn init_strategy_config(&mut self, config: Self::Config);

    fn handle_data(
        &mut self,
        market_data: Self::MarketData,
        indicators: Indicators,
    ) -> anyhow::Result<OrderRequest>;

    /// 尝试平仓前要做前置校验，如时间、配置、阈值校验。
    fn pre_valid(&self) -> anyhow::Result<(), PreValidError>;

    /// 后置校验，如仓位、资金的校验。
    fn rear_valid(&self) -> anyhow::Result<(), RearValidError>;

    /// 尝试开仓。
    fn try_open(&self) -> anyhow::Result<OrderRequest, OpenError>;

    /// 尝试平仓，平仓优先级大于开仓。
    fn try_close(&self) -> anyhow::Result<OrderRequest, CloseError>;

    fn build_order(
        &self,
        side: Side,
        market_data: &Self::MarketData,
        indicators: &Indicators,
        config: &Self::Config,
    ) -> anyhow::Result<OrderRequest>;

    fn common_handler(&self) -> anyhow::Result<OrderRequest> {
        let handle = || {
            self.pre_valid()?;
            let order_req = self.try_close().or_else(|_| self.try_open())?;
            self.rear_valid()?;
            anyhow::Ok(order_req)
        };
        handle()
    }
}
