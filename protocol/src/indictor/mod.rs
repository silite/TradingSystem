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
pub struct BundleMarketIndicator<D: OHLCV = Kline> {
    pub market_data: D,
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
impl<D: OHLCV + 'static> OHLCV for BundleMarketIndicator<D> {
    fn open(&self) -> yata::core::ValueType {
        self.market_data.open()
    }

    fn high(&self) -> yata::core::ValueType {
        self.market_data.high()
    }

    fn low(&self) -> yata::core::ValueType {
        self.market_data.low()
    }

    fn close(&self) -> yata::core::ValueType {
        self.market_data.close()
    }

    fn volume(&self) -> yata::core::ValueType {
        self.market_data.volume()
    }
}
