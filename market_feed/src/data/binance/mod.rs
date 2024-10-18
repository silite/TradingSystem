use derive_builder::Builder;
use protocol::portfolio::market_data::binance::Kline;

use crate::MarketFeed;

#[derive(Builder, Clone)]
pub struct BinanceMarketFeed {}

impl MarketFeed for BinanceMarketFeed {
    type MarketData = Kline;

    // TODO + ?
    type BundleData = Kline;

    async fn load_history_market_data(&mut self, data: &Self::MarketData) -> anyhow::Result<()> {
        todo!()
    }

    async fn handle_market_data(&mut self, data: &Self::MarketData) -> anyhow::Result<()> {
        todo!()
    }

    fn is_linked(&self) -> bool {
        todo!()
    }

    fn computed_indicator(&mut self) {
        todo!()
    }

    fn subscribe(&self) -> anyhow::Result<Self::BundleData> {
        todo!()
    }
}
