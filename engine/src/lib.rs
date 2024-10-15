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
use protocol::{command::Command, market::Market};
use strategy::StrategyExt;
use trader::Trader;

#[derive(Builder)]
pub struct Engine<Portfolio, MarketData, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketData: MarketFeed,
    Strategy: StrategyExt,
{
    /// 全局唯一engine_id，索引全局唯一的portfolio，掌管一批traders。
    engine_id: uuid::Uuid,
    /// 响应命令的管线。
    command_rx: crossbeam::channel::Receiver<Command>,
    /// 启动时全局静态配置/环境变量。
    config: conf::Config,
    /// 全局投资组合，每个trader独占享有的资金，每个trader的资产更新时，会异步patch更新到全局。
    /// 所以这里的数据更新可能并不及时。
    portfolio: HashMap<Market, Portfolio>,
    /// 每个traders享有独占的账户资产，trader 和 strategy 为1对1。
    traders: HashMap<Market, Trader<Portfolio, MarketData, Execution, Strategy>>,
}

impl<Portfolio, MarketData, Execution, Strategy> Engine<Portfolio, MarketData, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    MarketData: MarketFeed,
    Strategy: StrategyExt,
{
}
