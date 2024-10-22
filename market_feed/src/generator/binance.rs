use protocol::portfolio::market_data::binance::Kline;
use tokio::sync::mpsc;

use crate::{data::binance::BinanceMarketFeed, MarketFeed};

use super::{Feed, MarketGenerator};

#[derive(Clone)]
pub struct BinanceMarketGenerator<Event> {
    market_data_rx: crossbeam::channel::Receiver<Event>,
}

impl<Event> MarketGenerator<Event> for BinanceMarketGenerator<Event> {
    fn new(market_data_rx: crossbeam::channel::Receiver<Event>) -> Self {
        Self { market_data_rx }
    }

    fn next(&mut self) -> super::Feed<Event> {
        loop {
            match self.market_data_rx.try_recv() {
                Ok(event) => break Feed::Next(event),
                Err(crossbeam::channel::TryRecvError::Empty) => continue,
                Err(crossbeam::channel::TryRecvError::Disconnected) => break Feed::Finished,
            }
        }
    }
}
