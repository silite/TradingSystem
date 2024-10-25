use std::{sync::Arc, thread::JoinHandle};

use derive_builder::Builder;
use protocol::{
    event::bus::CommandBus, indictor::Indicators, portfolio::market_data::binance::Kline,
    trade::Side,
};
use yata::core::OHLCV;

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
    indicators: Option<Indicators>,
    market_data: Option<Kline>,
    config: MacdStrategyConfig,
    state: MacdStrategyState,
}

impl StrategyExt for MacdStrategy {
    type MarketData = Kline;
    type Config = MacdStrategyConfig;

    fn init_strategy_config(&mut self, config: Self::Config) {
        self.config = config;
    }

    // fn run(mut self, command_bus: Arc<CommandBus>) -> JoinHandle<anyhow::Result<()>> {
    // ftlog::info!("[strategy] macd strategy run.");
    // let command_rs = command_bus.subscribe(self.market_feed_topic.to_owned());
    // std::thread::spawn(move || {
    //     while let Some(event) = command_rs.recv() {
    //         match event {
    //             protocol::event::Event::MarketData(market_data_event) => {
    //                 match market_data_event.kind {
    //                     protocol::event::DataKind::Kline(kline) => todo!(),
    //                     protocol::event::DataKind::BundleData((market_data, indicators)) => {
    //                         self.handle_data(market_data, indicators);
    //                     }
    //                 }
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    //     Ok(())
    // })
    // }

    fn pre_valid(&self) -> anyhow::Result<(), crate::error::PreValidError> {
        Ok(())
    }

    fn rear_valid(&self) -> anyhow::Result<(), crate::error::RearValidError> {
        Ok(())
    }

    fn try_open(&self) -> anyhow::Result<(), OpenError> {
        let market_data = self.market_data.as_ref().unwrap();
        let indicators = self.indicators.as_ref().unwrap();
        let config = &self.config;

        if market_data.close_time - self.state.last_open_ts <= config.open_interval {
            return Err(OpenError::OpenQuickSuccession);
        }
        if indicators.adx <= config.adx_threshold {
            return Err(OpenError::ADXInsufficient(indicators.adx));
        }
        let (k, d) = (indicators.stoch_rsi.k, indicators.stoch_rsi.d);
        let macd_diff = indicators.macd.0 - indicators.macd.1;
        let last_macd_diff: f64 = self.state.last_macd_diff;
        let stoch_rsi_diff = k - d;
        let last_stoch_rsi_diff = self.state.last_stoch_rsi_diff;
        if indicators.macd.0 > indicators.macd.1
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
        if indicators.macd.0 < indicators.macd.1
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
            indicators.macd.0,
            indicators.macd.1,
            market_data.low,
            market_data.high,
        ))
    }

    fn try_close(&self) -> anyhow::Result<(), CloseError> {
        Err(CloseError::Test)
    }

    fn handle_data(&mut self, market_data: Self::MarketData, indicators: Indicators) {
        let stoch_rsi_diff = indicators.stoch_rsi.k - indicators.stoch_rsi.d;
        let macd_diff = indicators.macd.0 - indicators.macd.1;

        self.indicators = Some(indicators);
        self.market_data = Some(market_data);
        self.common_handler();

        self.state.last_stoch_rsi_diff = stoch_rsi_diff;
        self.state.last_macd_diff = macd_diff;
    }
}
