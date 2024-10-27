#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod machine;

use std::{
    collections::VecDeque,
    marker::PhantomData,
    ops::Deref,
    os::unix::process::ExitStatusExt,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use execution::ExecutionExt;
use market_feed::{indictor, MarketFeed};
use portfolio::{balance::BalanceHandler, error::PortfolioError, position::PositionHandler};
use protocol::{
    event::{bus::CommandBus, TradeEvent},
    indictor::Indicators,
    market::Market,
    order::OrderResponse,
};
use strategy::StrategyExt;
use utils::runtime::TOKIO_RUNTIME;
use yata::core::OHLCV;

#[derive(Builder, Clone)]
pub struct Trader<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler,
    Strategy: StrategyExt,
    Execution: ExecutionExt,
{
    /// Used as a unique identifier seed for the Portfolio, Trader & Positions associated with this [`Engine`].
    engine_id: uuid::Uuid,
    /// 交易所标的
    market: Market,
    /// receiving [`Command`]s from a remote source.
    command_bus: Arc<CommandBus>,
    /// 资券
    portfolio: Portfolio,
    /// 行情、指标源
    market_feed_rx:
        crossbeam::channel::Receiver<(<Strategy as StrategyExt>::MarketData, Indicators)>,
    /// 执行器
    execution: Arc<Execution>,
    /// 策略
    strategy: Strategy,
    /// 事件循环队列
    command_queue: SegQueue<TradeEvent<<Strategy as StrategyExt>::MarketData>>,
}

impl<Portfolio, Execution, Strategy> Trader<Portfolio, Execution, Strategy>
where
    Portfolio: BalanceHandler + PositionHandler + Send + 'static,
    Strategy: StrategyExt + Send + 'static,
    Execution: ExecutionExt + Send + Sync + 'static,
{
    /// trader.run时，策略也要run，监听事件。market_feed.run晚于strategy.run。
    pub async fn run(self) -> anyhow::Result<()> {
        ftlog::info!("[trade] {} {:?} run.", self.engine_id, self.market);
        Ok(std::thread::spawn(move || self.event_loop())
            .join()
            // 必须执行成功
            .unwrap()?)
    }

    /// 事件循环
    pub fn event_loop(mut self) -> anyhow::Result<()> {
        let (order_channel_tx, order_channel_rx) = crossbeam::channel::unbounded();
        loop {
            // FIXME 将market_feed移到单独线程
            match self.market_feed_rx.recv() {
                Ok(market) => {
                    self.command_queue.0.push(TradeEvent::Market(market));
                }
                Err(err) => {
                    ftlog::error!("[Trader Event Error] recv market data error. {}", err);
                }
            }

            // FIXME 将order_resp移到单独线程
            match order_channel_rx.try_recv() {
                Ok(Ok(order_resp)) => {
                    self.command_queue
                        .0
                        .push(TradeEvent::OrderUpdate(order_resp));
                }
                Ok(Err(err)) => {
                    ftlog::error!(
                        "[Trader Event Error] new order error. this is execution bug. {}",
                        err
                    );
                }
                _ => {}
            }

            // FIXME 将event_loop移到单独线程
            if let Some(event) = self.command_queue.0.pop() {
                match event {
                    TradeEvent::Market((market_data, indicators)) => {
                        if let Ok(order_req) = self.strategy.handle_data(market_data, indicators) {
                            self.command_queue.0.push(TradeEvent::OrderNew(order_req));
                        } else {
                            // 策略尝试开单失败
                        }
                    }
                    TradeEvent::OrderNew(order_request) => {
                        // 锁资
                        if let Some(price) = order_request.main_order.price {
                            let amount = price * order_request.main_order.volume;
                            if let Err(err) = self.portfolio.diff_open_freezed_balance(amount) {
                                ftlog::error!("[Trade Event Error] OrderNew error. {}", err);
                                continue;
                            }
                        }

                        // 下单
                        let execution = self.execution.clone();
                        let order_channel_tx = order_channel_tx.clone();
                        TOKIO_RUNTIME.spawn(async move {
                            if let Err(err) =
                                execution.new_order(order_request, order_channel_tx).await
                            {
                                ftlog::error!("[Trade Event Error] OrderNew error. {}", err);
                            }
                        });
                    }
                    TradeEvent::OrderUpdate(order_resp) => match order_resp {
                        OrderResponse::OrderSuccess((amount, volume)) => {
                            if let Err(err) = self.portfolio.diff_freezed_balance(-amount) {
                                ftlog::error!("[Trade Event Error] OrderNew error. {}", err);
                                continue;
                            } else {
                            }
                        }
                        OrderResponse::OrderError((amount, volume)) => {
                            if let Err(err) = self.portfolio.diff_open_freezed_balance(-amount) {
                                ftlog::error!("[Trade Event Error] OrderNew error. {}", err);
                                continue;
                            }
                        }
                    },
                }
            }
        }
    }
}

/// "WARNING", 仅为了Builder，不是真实Clone
pub struct SegQueue<T>(crossbeam_queue::SegQueue<T>);
impl<T> Clone for SegQueue<T> {
    fn clone(&self) -> Self {
        SegQueue(crossbeam_queue::SegQueue::new())
    }
}

impl<T> Default for SegQueue<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
