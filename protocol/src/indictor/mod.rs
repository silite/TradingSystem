use yata::core::OHLCV;

use crate::portfolio::market_data::binance::Kline;

#[derive(Debug, Clone)]
pub struct DCData {
    pub high: f64,
    pub low: f64,
    pub mid: f64,
}

#[derive(Debug, Clone)]
pub struct StochRsiDATA {
    pub k: f64,
    pub d: f64,
}

#[derive(Debug, Clone)]
pub struct Indicators {
    pub dc: DCData,
    pub rsi: f64,
    pub ema: f64,
    pub stoch_rsi: StochRsiDATA,
    pub adx: f64,
    pub macd: (f64, f64),
    pub tr_rma: f64,
    pub tr: f64,
    pub atr: (f64, f64),
}
