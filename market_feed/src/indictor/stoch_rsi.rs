use protocol::indictor::StochRsiDATA;

use super::Indicator;

const DEFAULT_LENGTH: usize = 14;

#[derive(Clone)]
pub struct StochRSI {
    rsi_list: circular_queue::CircularQueue<f64>,
    rsi_d_list: circular_queue::CircularQueue<f64>,
}

impl StochRSI {
    pub fn new(length: Option<usize>) -> Self {
        Self {
            rsi_list: circular_queue::CircularQueue::with_capacity(
                length.unwrap_or(DEFAULT_LENGTH),
            ),
            rsi_d_list: circular_queue::CircularQueue::with_capacity(3),
        }
    }

    pub fn push(&mut self, rsi: f64) -> &mut Self {
        self.rsi_list.push(rsi);
        self
    }

    pub fn get(&mut self, rsi: f64) -> StochRsiDATA {
        let (high, low): (f64, f64) = self.rsi_list.iter().fold((0., f64::MAX), |mut res, curr| {
            res.0 = res.0.max(*curr);
            res.1 = res.1.min(*curr);
            res
        });
        let stoch_rsi = if high - low == 0. {
            0.
        } else {
            (rsi - low) / (high - low)
        };
        self.rsi_d_list.push(stoch_rsi);
        StochRsiDATA {
            k: stoch_rsi,
            d: self.rsi_d_list.iter().sum::<f64>() / 3.,
        }
    }

    pub fn next(&mut self, rsi: f64) -> StochRsiDATA {
        self.push(rsi).get(rsi)
    }
}
