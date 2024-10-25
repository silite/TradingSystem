use std::{collections::HashMap, marker::PhantomData};

use chrono::{DateTime, Utc};
use dashmap::DashMap;

use crate::{
    indictor::Indicators,
    market::{exchange::Exchange, symbol::Instrument},
    portfolio::market_data::binance::Kline,
};

#[derive(Debug)]
pub enum Event {
    MarketData(MarketDataEvent),
    MarketFeed(MarketFeedEvent),
    TradeExecution(),
    PortfolioUpdate(),
    Command(CommandEvent),
}

/// 事件总线，解耦各个模块，并异步处理事件
pub struct EventBus {
    senders: DashMap<String, crossbeam::channel::Sender<Event>>,
    senders_sync: DashMap<String, tokio::sync::mpsc::UnboundedSender<Event>>,
}
impl EventBus {
    pub fn new() -> Self {
        EventBus {
            senders: DashMap::new(),
            senders_sync: DashMap::new(),
        }
    }

    pub fn subscribe(&self, topic: String) -> crossbeam::channel::Receiver<Event> {
        let (sender, receiver) = crossbeam::channel::unbounded();
        self.senders.insert(topic, sender);
        receiver
    }

    pub fn subscribe_sync(&self, topic: String) -> tokio::sync::mpsc::UnboundedReceiver<Event> {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        self.senders_sync.insert(topic, sender);
        receiver
    }

    pub fn publish(&self, topic: &str, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Command(_) | Event::MarketFeed(_) => {
                if let Some(sender) = self.senders_sync.get(topic) {
                    sender.send(event)?
                } else {
                    ftlog::error!("PLACE FIXME. {} {:?}", topic, event);
                }
            }
            Event::MarketData(_) | Event::TradeExecution() | Event::PortfolioUpdate() => {
                if let Some(sender) = self.senders.get(topic) {
                    sender.send(event)?
                } else {
                    ftlog::error!("PLACE FIXME. {} {:?}", topic, event);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CommandEvent {
    Terminate(String),
}

/// Normalised Barter [`MarketEvent<T>`](Self) wrapping the `T` data variant in metadata.
///
/// Note: `T` can be an enum such as the [`DataKind`] if required.
///
/// See [`crate::subscription`] for all existing Barter Market event variants.
///
/// ### Examples
/// - [`MarketEvent<PublicTrade>`](PublicTrade)
/// - [`MarketEvent<OrderBookL1>`](OrderBookL1)
/// - [`MarketEvent<DataKind>`](DataKind)
// pub struct MarketEvent<InstrumentId = Instrument, T = DataKind> {
#[derive(Debug, Clone)]
pub struct MarketDataEvent<T = DataKind> {
    // pub exchange_time: DateTime<Utc>,
    // pub received_time: DateTime<Utc>,
    // pub exchange: Exchange,
    // pub instrument: InstrumentId,
    pub kind: T,
}

/// Available kinds of normalised Barter [`MarketEvent<T>`](MarketEvent).
///
/// ### Notes
/// - [`Self`] is only used as the [`MarketEvent<DataKind>`](MarketEvent) `Output` when combining
///   several [`Streams<SubscriptionKind::Event>`](crate::streams::Streams) using the
///   [`MultiStreamBuilder<Output>`](crate::streams::builder::multi::MultiStreamBuilder), or via
///   the [`DynamicStreams::select_all`](crate::streams::builder::dynamic::DynamicStreams) method.
/// - [`Self`] is purposefully not supported in any
///   [`Subscription`](crate::subscription::Subscription)s directly, it is only used to
///   make ergonomic [`Streams`](crate::streams::Streams) containing many
///   [`MarketEvent<T>`](MarketEvent) kinds.
#[derive(Debug, Clone)]
pub enum DataKind {
    Kline(Kline),
    BundleData((Kline, Indicators)),
}

#[derive(Debug)]
pub enum MarketFeedEvent {
    /// 读取历史所有行情
    LoadHistory,
}
