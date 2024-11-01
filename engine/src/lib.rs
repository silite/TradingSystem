#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    collections::HashMap,
    os::unix::process::ExitStatusExt,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use execution::ExecutionExt;
use market_feed::MarketFeed;
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{event::bus::CommandBus, market::Market};
use rayon::prelude::*;
use strategy::StrategyExt;
use trader::Trader;
use utils::runtime::TOKIO_RUNTIME;

#[derive(Builder)]
pub struct Engine<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    Strategy: StrategyExt,
    Execution: ExecutionExt,
{
    /// 全局唯一engine_id，索引全局唯一的portfolio，掌管一批traders。
    engine_id: uuid::Uuid,
    /// 响应命令的管线。
    command_bus: Arc<CommandBus>,
    /// 启动时全局静态配置/环境变量。
    config: conf::Config,
    /// 全局投资组合，每个trader独占享有的资金，每个trader的资产更新时，会异步patch更新到全局;
    /// 所以这里的数据更新可能并不及时;
    /// 因为portfolio可clone，所以透传给trader作为初始化值，但trader内部更新不需要锁。
    portfolio: HashMap<Market, Portfolio>,
    /// 每个traders享有独占的账户资产，trader 和 strategy 为1对1。
    traders: HashMap<Market, Trader<Portfolio, Execution, Strategy>>,
}

impl<Portfolio, Execution, Strategy> Engine<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler + Send + 'static,
    Strategy: StrategyExt + Send + 'static,
    Execution: ExecutionExt + Send + Sync + 'static,
{
    /// engine托管多个trader，engine.run时执行每个trader的run
    pub fn run(self) -> anyhow::Result<()> {
        ftlog::info!("[engine] {} run.", self.engine_id);
        self.traders.into_iter().for_each(|(market, trade)| {
            TOKIO_RUNTIME.spawn(async move {
                let _ = trade
                    .run()
                    .await
                    .map_err(|err| ftlog::error!("[trade] run error. {:?}", err));
            });
        });
        Ok(())
    }
}
