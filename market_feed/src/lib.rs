#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::process::Command;

use protocol::event::MarketEvent;
use tokio::sync::mpsc;
use yata::core::OHLCV;

pub mod data;
pub mod generator;
pub mod indictor;

/// 基于划线来做策略的Feed方案。
/// 先`接收互联网行情`，保证一直接收最新数据。然后`从头load所有历史数据`接上断点后，行情才能连续，保证使用正确。
#[allow(async_fn_in_trait)]
pub trait MarketFeed: Sized {
    /// 原始行情
    type MarketData: OHLCV;

    type Command;

    // 返回指令接受 和 数据推送channel
    fn new() -> (
        mpsc::UnboundedSender<Self::Command>,
        crossbeam::channel::Receiver<MarketEvent>,
    );

    /// 相应command
    async fn run(self) -> anyhow::Result<()>;

    /// 从头load所有历史行情，因为指标需要从头开始计算
    /// 这里优雅停机后，可以将需要的指标缓存，这样可以断点恢复
    /// 这里的数据不会保留在内存
    async fn load_history_market_data(&mut self) -> anyhow::Result<i32>;

    /// 接收互联网行情，在断点跟旧数据链接之前，数据会一直保留在内存
    async fn handle_market_data(&mut self) -> anyhow::Result<()>;

    /// 历史行情 与 新行情是否链接上
    fn is_linked(&self) -> bool;

    /// 计算指标
    fn computed_indicator(&mut self, data: &Self::MarketData);
}
