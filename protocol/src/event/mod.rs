use std::{collections::HashMap, marker::PhantomData};

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use yata::core::OHLCV;

use crate::{
    indictor::Indicators,
    market::{exchange::Exchange, symbol::Instrument},
    portfolio::market_data::binance::Kline,
};

pub mod bus;

#[derive(Debug, Clone)]
pub enum TradeEvent<MarketData: OHLCV> {
    Market((MarketData, Indicators)),
}

#[derive(Debug)]
pub enum Command {
    Terminate(String),
    MarketFeed(MarketFeedCommand),
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
pub enum MarketFeedCommand {
    /// 读取历史所有行情
    LoadHistory,
}
