use yata::core::{Candle, Method, OHLCV};

use super::Indicator;

#[derive(Clone)]
pub struct TR {
    quote: yata::methods::TR,
    tr: f64,
}

impl Indicator for TR {
    type Output = f64;

    fn new(_config: Self::Config) -> Self {
        Self {
            quote: yata::methods::TR::new(&Candle::default()).unwrap(),
            tr: 0.,
        }
    }

    fn push(&mut self, kline: &impl OHLCV) -> &mut Self {
        self.tr = self.quote.next(&Candle::from(kline));
        self
    }

    fn get(&self) -> f64 {
        self.tr
    }
}
