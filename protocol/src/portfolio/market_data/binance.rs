use clickhouse::Row;
use yata::core::OHLCV;

#[derive(Row, Clone)]
pub struct Kline {
    pub open_time: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: u64,
    pub quote_asset_volume: f64,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

impl OHLCV for Kline {
    fn open(&self) -> yata::core::ValueType {
        self.open
    }

    fn high(&self) -> yata::core::ValueType {
        self.high
    }

    fn low(&self) -> yata::core::ValueType {
        self.low
    }

    fn close(&self) -> yata::core::ValueType {
        self.close
    }

    fn volume(&self) -> yata::core::ValueType {
        self.volume
    }
}
