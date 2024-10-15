use circular_queue::CircularQueue;
use yata::core::OHLCV;

use super::Indicator;

const DEFAULT_LENGTH: usize = 20;

#[derive(Debug)]
pub struct DCData {
    pub high: f64,
    pub low: f64,
    pub mid: f64,
}

pub struct DC {
    quotes: CircularQueue<DCData>,
}

impl Indicator for DC {
    type Output = DCData;
    type Config = Option<usize>;

    fn new(length: Option<usize>) -> Self {
        let length = length.unwrap_or(DEFAULT_LENGTH);
        Self {
            quotes: CircularQueue::with_capacity(length),
        }
    }

    fn push(&mut self, k: &impl OHLCV) -> &mut Self {
        let dc_data = DCData {
            high: k.high(),
            low: k.low(),
            mid: 0.,
        };
        self.quotes.push(dc_data);
        self
    }

    fn get(&self) -> DCData {
        let (high, low): (f64, f64) = self.quotes.iter().fold((0., f64::MAX), |mut res, curr| {
            res.0 = res.0.max(curr.high);
            res.1 = res.1.min(curr.low);
            res
        });
        DCData {
            high,
            low,
            mid: (high + low) / 2.,
        }
    }
}
