#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use error::{CloseError, OpenError, PreValidError, RearValidError};
use protocol::event::MarketEvent;
use signal::SignalExt;

mod error;
pub mod implements;
mod signal;

pub trait StrategyExt {
    /// 接收事件源
    fn handle_event(
        self,
        rx: crossbeam::channel::Receiver<MarketEvent>,
    ) -> std::thread::JoinHandle<anyhow::Result<()>>;

    /// 尝试平仓前要做前置校验，如时间、配置、阈值校验。
    fn pre_valid(&self) -> anyhow::Result<(), PreValidError>;

    /// 后置校验，如仓位、资金的校验。
    fn rear_valid(&self) -> anyhow::Result<(), RearValidError>;

    /// 尝试开仓。
    fn try_open(&self) -> anyhow::Result<(), OpenError>;

    /// 尝试平仓，平仓优先级大于开仓。
    fn try_close(&self) -> anyhow::Result<(), CloseError>;
}
