use yata::core::Method;

use super::Indicator;

pub const LENGTH: u8 = 14;

pub struct TrRMA {
    quote: yata::methods::RMA,
    tr_rma: f64,
}

impl TrRMA {
    pub fn new() -> Self {
        Self {
            quote: yata::methods::RMA::new(LENGTH, &0.).unwrap(),
            tr_rma: 0.,
        }
    }

    pub fn push(&mut self, tr: f64) -> &mut Self {
        self.tr_rma = self.quote.next(&tr);
        self
    }

    pub fn get(&self) -> f64 {
        self.tr_rma
    }

    pub fn next(&mut self, tr: f64) -> f64 {
        self.push(tr).get()
    }
}
