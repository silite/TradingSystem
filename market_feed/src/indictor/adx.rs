use yata::{
    core::{Candle, IndicatorConfig, IndicatorInstanceDyn, OHLCV},
    indicators::{AverageDirectionalIndex, AverageDirectionalIndexInstance},
};

use super::Indicator;

pub struct ADX {
    quote: AverageDirectionalIndexInstance,
    data: f64,
}

impl Indicator for ADX {
    type Output = f64;

    fn new(_config: Self::Config) -> Self {
        Self {
            quote: AverageDirectionalIndex::init(
                AverageDirectionalIndex::default(),
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
