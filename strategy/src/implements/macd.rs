use std::{sync::Arc, thread::JoinHandle};

use derive_builder::Builder;
use protocol::{event::EventBus, indictor::BundleMarketIndicator, trade::Side};

use crate::{
    error::{CloseError, OpenError},
    StrategyExt,
};

#[derive(Clone, Debug)]
pub struct MacdStrategyConfig {
    pub open_interval: u64,
    pub adx_threshold: f64,
    pub macd_diff: f64,
    pub rsi_diff: f64,
}

#[derive(Debug, Default, Clone)]
pub struct MacdStrategyState {
    last_open_ts: u64,
    last_stoch_rsi_diff: f64,
    last_macd_diff: f64,
}

#[derive(Builder, Clone)]
pub struct MacdStrategy {
    market_feed_topic: &'static str,
    bundle_market_indicator: Option<BundleMarketIndicator>,
    config: MacdStrategyConfig,
    state: MacdStrategyState,
}

impl StrategyExt for MacdStrategy {
    type Config = MacdStrategyConfig;

    fn init_strategy_config(&mut self, config: Self::Config) {
        self.config = config;
    }

    fn run(mut self, event_bus: Arc<EventBus>) -> JoinHandle<anyhow::Result<()>> {
        ftlog::info!("[strategy] macd strategy run.");
        let event_rs = event_bus.subscribe(self.market_feed_topic.to_owned());
        std::thread::spawn(move || {
            while let Ok(event) = event_rs.recv() {
                match event {
                    protocol::event::Event::MarketData(market_data_event) => {
                        match market_data_event.kind {
                            protocol::event::DataKind::Kline(kline) => todo!(),
                            protocol::event::DataKind::BundleData(bundle_market_indicator) => {
                                let stoch_rsi_diff = bundle_market_indicator.stoch_rsi.k
                                    - bundle_market_indicator.stoch_rsi.d;
                                let macd_diff =
                                    bundle_market_indicator.macd.0 - bundle_market_indicator.macd.1;
                                self.bundle_market_indicator = Some(bundle_market_indicator);
                                self.common_handler();
                                self.state.last_stoch_rsi_diff = stoch_rsi_diff;
                                self.state.last_macd_diff = macd_diff;
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
        Ok(())
    }

    fn rear_valid(&self) -> anyhow::Result<(), crate::error::RearValidError> {
        Ok(())
    }

    fn try_open(&self) -> anyhow::Result<(), OpenError> {
        let (quote, indicator) =
            if let Some(bundle_market_indicator) = &self.bundle_market_indicator {
                (
                    &bundle_market_indicator.market_data,
                    bundle_market_indicator,
                )
            } else {
                panic!()
            };
        let config = &self.config;

        if quote.close_time - self.state.last_open_ts <= config.open_interval {
            return Err(OpenError::OpenQuickSuccession);
        }
        if indicator.adx <= config.adx_threshold {
            return Err(OpenError::ADXInsufficient(indicator.adx));
        }
        let (k, d) = (indicator.stoch_rsi.k, indicator.stoch_rsi.d);
        let macd_diff = indicator.macd.0 - indicator.macd.1;
        let last_macd_diff: f64 = self.state.last_macd_diff;
        let stoch_rsi_diff = k - d;
        let last_stoch_rsi_diff = self.state.last_stoch_rsi_diff;
        if indicator.macd.0 > indicator.macd.1
            && macd_diff > config.macd_diff
            && ((last_macd_diff * macd_diff > 0. && macd_diff > last_macd_diff)
                || last_macd_diff * macd_diff < 0.)
        {
            if k > d
                && stoch_rsi_diff > config.rsi_diff
                && last_stoch_rsi_diff * stoch_rsi_diff > 0.
                && stoch_rsi_diff > last_stoch_rsi_diff
            {
                return Ok(());
            }
            return Err(OpenError::StochRSIOversoldIncompatible(k, d));
        }
        if indicator.macd.0 < indicator.macd.1
            && -macd_diff > config.macd_diff
            && macd_diff * last_macd_diff > 0.
            && macd_diff.abs() > last_macd_diff.abs()
        {
            if d > k
                && -stoch_rsi_diff > config.rsi_diff
                && ((last_stoch_rsi_diff * stoch_rsi_diff > 0.
                    && stoch_rsi_diff.abs() > last_stoch_rsi_diff.abs())
                    || last_macd_diff * stoch_rsi_diff < 0.)
            {
                return Ok(());
            }
            return Err(OpenError::StochRSIOverboughtIncompatible(k, d));
        }

        Err(OpenError::MACDIncompatible(
            indicator.macd.0,
            indicator.macd.1,
            quote.low,
            quote.high,
        ))
    }

    fn try_close(&self) -> anyhow::Result<(), CloseError> {
        Err(CloseError::Test)
    }
}
