use std::marker::PhantomData;

use chrono::{DateTime, Utc};

use crate::{
    indictor::BundleMarketIndicator,
    market::{exchange::Exchange, symbol::Instrument},
    portfolio::market_data::binance::Kline,
};

pub enum Command {
    /// Terminate every running [`Trader`] associated with this [`Engine`]. Involves all [`Trader`]s.
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
pub struct MarketEvent<T = DataKind> {
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
    BundleData(BundleMarketIndicator),
}
