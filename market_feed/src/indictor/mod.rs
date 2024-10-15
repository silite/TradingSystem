use yata::core::OHLCV;

mod adx;
mod dc;
mod ema;
mod macd;
mod rsi;
mod stoch_rsi;
mod tr;
mod tr_rma;

pub trait Indicator {
    type Output;
    type Config = ();

    ///
    fn new(config: Self::Config) -> Self;
    ///
    fn push(&mut self, k: &impl OHLCV) -> &mut Self;
    ///
    fn get(&self) -> Self::Output;
    ///
    fn next(&mut self, k: &impl OHLCV) -> Self::Output {
        self.push(k).get()
    }
}
