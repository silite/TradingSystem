use std::{sync::Arc, thread::JoinHandle};

use derive_builder::Builder;
use protocol::{event::EventBus, indictor::BundleMarketIndicator};

use crate::StrategyExt;

#[derive(Builder, Clone)]
pub struct MacdStrategy {
    market_feed_topic: &'static str,
    bundle_market_indicator: BundleMarketIndicator,
    last_open_ts: u64,
    last_stoch_rsi_diff: f64,
    open_interval: u64,
    adx_threshold: f64,

    last_macd_diff: f64,
    macd_diff: f64,
    rsi_diff: f64,
}

impl StrategyExt for MacdStrategy {
    fn run(mut self, event_bus: Arc<EventBus>) -> JoinHandle<anyhow::Result<()>> {
        let event_rs = event_bus.subscribe(self.market_feed_topic.to_owned());
        std::thread::spawn(move || {
            println!("{:?}", 456);
            while let Ok(event) = event_rs.recv() {
                match event {
                    protocol::event::Event::MarketData(market_data_event) => {
                        match market_data_event.kind {
                            protocol::event::DataKind::Kline(kline) => todo!(),
                            protocol::event::DataKind::BundleData(bundle_market_indicator) => {
                                self.bundle_market_indicator = bundle_market_indicator;
                                self.pre_valid()?;
                                self.try_close()?;
                                self.try_open()?;
                                self.rear_valid()?;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Ok(())
        })
    }

    fn pre_valid(&self) -> anyhow::Result<(), crate::error::PreValidError> {
        todo!()
    }

    fn rear_valid(&self) -> anyhow::Result<(), crate::error::RearValidError> {
        todo!()
    }

    fn try_open(&self) -> anyhow::Result<(), crate::error::OpenError> {
        let quote = &self.bundle_market_indicator.market_data;
        let indicator = &self.bundle_market_indicator;
        if quote.close_time - self.last_open_ts <= self.open_interval {
            // return Err(OpenFail::OpenQuickSuccession);
        }
        if self.bundle_market_indicator.adx <= self.adx_threshold {
            // return Err(OpenFail::ADXInsufficient(quote.adx));
        }
        let (k, d) = (indicator.stoch_rsi.k, indicator.stoch_rsi.d);
        let macd_diff = indicator.macd.0 - indicator.macd.1;
        let last_macd_diff: f64 = self.last_macd_diff;
        let stoch_rsi_diff = k - d;
        let last_stoch_rsi_diff = self.last_stoch_rsi_diff;
        if indicator.macd.0 > indicator.macd.1
            && macd_diff > self.macd_diff
            && ((last_macd_diff * macd_diff > 0. && macd_diff > last_macd_diff)
                || last_macd_diff * macd_diff < 0.)
        {
            if k > d
                && stoch_rsi_diff > self.rsi_diff
                && last_stoch_rsi_diff * stoch_rsi_diff > 0.
                && stoch_rsi_diff > last_stoch_rsi_diff
            {
                // return Ok(OrderType::OpenLong);
            }
            // return Err(OpenFail::StochRSIOversoldIncompatible(k, d));
        }
        if indicator.macd.0 < indicator.macd.1
            && -macd_diff > self.macd_diff
            && macd_diff * last_macd_diff > 0.
            && macd_diff.abs() > last_macd_diff.abs()
        {
            if d > k
                && -stoch_rsi_diff > self.rsi_diff
                && ((last_stoch_rsi_diff * stoch_rsi_diff > 0.
                    && stoch_rsi_diff.abs() > last_stoch_rsi_diff.abs())
                    || last_macd_diff * stoch_rsi_diff < 0.)
            {
                // return Ok(OrderType::OpenShort);
            }
            // return Err(OpenFail::StochRSIOverboughtIncompatible(k, d));
        }
        Ok(())

        // return Err(OpenFail::MACDIncompatible(
        //     quote.macd.0,
        //     quote.macd.1,
        //     quote.kline.low,
        //     quote.kline.high,
        // ));
    }

    fn try_close(&self) -> anyhow::Result<(), crate::error::CloseError> {
        todo!()
    }
}
