use yata::{
    core::{Candle, IndicatorConfig, IndicatorInstance, OHLCV},
    indicators::RelativeStrengthIndexInstance,
};

use super::Indicator;

#[derive(Clone)]
pub struct RSI {
    quote: RelativeStrengthIndexInstance,
    data: f64,
}

impl Indicator for RSI {
    type Output = f64;

    fn new(_config: Self::Config) -> Self {
        Self {
            quote: yata::indicators::RSI::init(
                yata::indicators::RSI::default(),
                &Candle::default(),
            )
            .unwrap(),
            data: 0.,
        }
    }

    fn push(&mut self, data: &impl OHLCV) -> &mut Self {
        self.data = self.quote.next(&Candle::from(data)).value(0);
        self
    }

    fn get(&self) -> f64 {
        self.data
    }
}
