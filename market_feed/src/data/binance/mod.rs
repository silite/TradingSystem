use crossbeam::channel::{Receiver, Sender};
use derive_builder::Builder;
use protocol::portfolio::market_data::binance::Kline;

use crate::{
    indictor::{BundleMarketIndicator, Indicator, IndicatorsCollection},
    MarketFeed,
};

#[derive(Clone)]
pub struct BinanceMarketFeed {
    indicators: IndicatorsCollection,
    subscribe_channel: Option<Sender<BundleMarketIndicator<Kline>>>,
}

impl MarketFeed for BinanceMarketFeed {
    type MarketData = Kline;

    // TODO + ?
    type BundleData = BundleMarketIndicator<Kline>;

    fn new() -> Self {
        Self {
            indicators: IndicatorsCollection::new(),
            subscribe_channel: None,
        }
    }

    async fn load_history_market_data(&mut self, data: &Self::MarketData) -> anyhow::Result<()> {
        todo!()
    }

    async fn handle_market_data(&mut self, data: &Self::MarketData) -> anyhow::Result<()> {
        todo!()
    }

    fn is_linked(&self) -> bool {
        todo!()
    }

    fn computed_indicator(&mut self, data: &Self::MarketData) {
        let dc = self.indicators.dc.push(data).get();
        let rsi = self.indicators.rsi.push(data).get();
        let ema = self.indicators.ema.push(data).get();
        let stoch_rsi = self.indicators.stock_rsi.push(rsi).get(rsi);
        let adx = self.indicators.adx.push(data).get();
        let macd = self.indicators.macd.push(data).get();
        let tr = self.indicators.tr.push(data).get();
        let tr_rma = self.indicators.tr_rma.push(tr).get();

        let atr_low = data.low - self.indicators.pre_tr_rma * 1.5;
        let atr_high = self.indicators.pre_tr_rma * 1.5 + data.high;
        self.indicators.pre_tr_rma = tr_rma;

        if let Some(sender) = &self.subscribe_channel {
            if let Err(e) = sender.send(BundleMarketIndicator {
                market_data: data.clone(),
                dc,
                rsi,
                ema,
                stoch_rsi,
                adx,
                macd,
                tr_rma,
                tr,
                atr: (atr_low, atr_high),
            }) {
                ftlog::error!("send computed indicator error {}", e);
            }
        }
    }

    fn subscribe(&mut self, sender: Sender<BundleMarketIndicator<Kline>>) -> anyhow::Result<()> {
        self.subscribe_channel = Some(sender);
        Ok(())
    }
}
