#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use market_feed::MarketFeed;
use portfolio::{balance::BalanceHandler, position::PositionHandler};
use protocol::{event::bus::CommandBus, market::Market};
use rayon::prelude::*;
use strategy::StrategyExt;
use trader::Trader;

#[derive(Builder)]
pub struct Engine<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    Strategy: StrategyExt,
    Execution: Send,
{
    /// 全局唯一engine_id，索引全局唯一的portfolio，掌管一批traders。
    engine_id: uuid::Uuid,
    /// 响应命令的管线。
    command_bus: Arc<CommandBus>,
    /// 启动时全局静态配置/环境变量。
    config: conf::Config,
    /// 全局投资组合，每个trader独占享有的资金，每个trader的资产更新时，会异步patch更新到全局。
    /// 所以这里的数据更新可能并不及时。
    portfolio: HashMap<Market, Portfolio>,
    /// 每个traders享有独占的账户资产，trader 和 strategy 为1对1。
    traders: HashMap<Market, Trader<Portfolio, Execution, Strategy>>,
}

impl<Portfolio, Execution, Strategy> Engine<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler + Send + 'static,
    Strategy: StrategyExt + Send + 'static,
    Execution: Send + 'static,
{
    /// engine托管多个trader，engine.run时执行每个trader的run
    pub fn run(self) -> anyhow::Result<()> {
        ftlog::info!("[engine] {} run.", self.engine_id);
        self.traders.into_iter().for_each(|(market, trade)| {
            tokio::spawn(async move {
                let _ = trade
                    .run()
                    .await
                    .map_err(|err| ftlog::error!("[trade] run error. {:?}", err));
            });
        });
        Ok(())
    }
}
