use yata::{
    core::{Candle, IndicatorConfig, IndicatorInstanceDyn, OHLCV},
    helpers::MA,
};

use super::Indicator;

#[derive(Clone)]
pub struct MACD {
    macd: yata::indicators::MACDInstance<MA>,
    data: (f64, f64),
}

impl Indicator for MACD {
    type Output = (f64, f64);

    fn new(_config: Self::Config) -> Self {
        let macd = yata::indicators::MACD::default();
        Self {
            macd: macd.init(&Candle::default()).unwrap(),
            data: (0., 0.),
        }
    }

    fn push(&mut self, data: &impl OHLCV) -> &mut Self {
        let res = self.macd.next(&Candle::from(data));
        let values = res.values();
        self.data = (values[0], values[1]);
        self
    }

    fn get(&self) -> (f64, f64) {
        self.data
    }
}
