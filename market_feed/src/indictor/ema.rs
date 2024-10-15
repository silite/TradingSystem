use yata::core::{Method, OHLCV};

use super::Indicator;

const DEFAULT_LENGTH: u8 = 200;

pub struct EMA {
    quote: yata::methods::EMA,
    data: f64,
}

impl Indicator for EMA {
    type Output = f64;
    type Config = Option<u8>;

    fn new(length: Option<u8>) -> Self {
        Self {
            quote: yata::methods::EMA::new(length.unwrap_or(DEFAULT_LENGTH), &0.).unwrap(),
            data: 0.,
        }
    }

    fn push(&mut self, data: &impl OHLCV) -> &mut Self {
        self.data = self.quote.next(&data.close());
        self
    }

    fn get(&self) -> f64 {
        self.data
    }
}
