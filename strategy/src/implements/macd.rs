use std::{sync::Arc, thread::JoinHandle};

use derive_builder::Builder;
use protocol::{
    event::bus::CommandBus,
    indictor::Indicators,
    order::{Order, OrderBuilder, OrderRequest, OrderType},
    portfolio::market_data::binance::Kline,
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
    pub atr_scaling: f64,
    /// 一手多少钱
    pub per_hand: f64,
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

    fn pre_valid(&self) -> anyhow::Result<(), crate::error::PreValidError> {
        Ok(())
    }

    fn rear_valid(&self) -> anyhow::Result<(), crate::error::RearValidError> {
        Ok(())
    }

    fn try_open(&self) -> anyhow::Result<OrderRequest, OpenError> {
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
                return Ok(self.build_order(Side::Buy, &market_data, &indicators, &self.config)?);
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
                return Ok(self.build_order(
                    Side::Sell,
                    &market_data,
                    &indicators,
                    &self.config,
                )?);
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

    fn try_close(&self) -> anyhow::Result<OrderRequest, CloseError> {
        Err(CloseError::Test)
    }

    // TODO 暂时Taker
    fn build_order(
        &self,
        side: Side,
        market_data: &Self::MarketData,
        indicators: &Indicators,
        config: &Self::Config,
    ) -> anyhow::Result<OrderRequest> {
        ftlog::info!(
            "[atr] {} {:?} {:?}",
            market_data.high,
            indicators.atr,
            config
        );
        let (price, atr) = if matches!(side, Side::Buy) {
            (
                market_data.high,
                (indicators.atr.0 * config.atr_scaling, indicators.atr.1),
            )
        } else {
            (
                market_data.low,
                (indicators.atr.1 * config.atr_scaling, indicators.atr.0),
            )
        };

        let volume = self.config.per_hand / price;
        Ok(OrderRequest {
            main_order: OrderBuilder::default()
                .side(side)
                .order_type(OrderType::Limit)
                .volume(volume)
                // TODO
                .price(Some(price))
                .build()?,
            take_profit: Some(
                OrderBuilder::default()
                    .side(!side)
                    .order_type(OrderType::Limit)
                    .volume(volume)
                    .price(Some(atr.0))
                    .build()?,
            ),
            stop_loss: Some(
                OrderBuilder::default()
                    .side(side)
                    .order_type(OrderType::Market)
                    .volume(volume)
                    .price(Some(atr.1))
                    .build()?,
            ),
        })
    }

    fn handle_data(
        &mut self,
        market_data: Self::MarketData,
        indicators: Indicators,
    ) -> anyhow::Result<OrderRequest> {
        let stoch_rsi_diff = indicators.stoch_rsi.k - indicators.stoch_rsi.d;
        let macd_diff = indicators.macd.0 - indicators.macd.1;

        self.indicators = Some(indicators);
        self.market_data = Some(market_data);

        let order_req = self.common_handler();

        self.state.last_stoch_rsi_diff = stoch_rsi_diff;
        self.state.last_macd_diff = macd_diff;

        order_req
    }
}
