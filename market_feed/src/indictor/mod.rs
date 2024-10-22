use adx::ADX;
use dc::DC;
use ema::EMA;
use macd::MACD;
use rsi::RSI;
use stoch_rsi::StochRSI;
use tr::TR;
use tr_rma::TrRMA;
use yata::core::OHLCV;

mod adx;
mod dc;
mod ema;
mod macd;
mod rsi;
mod stoch_rsi;
mod tr;
mod tr_rma;

/// 小指标的公共trait。
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

/// 指标集合。
/// FIXME 可能会有一些用不到的指标。
/// Clone only for Builder.
#[derive(Clone)]
pub struct IndicatorsCollection {
    pub dc: DC,
    pub rsi: RSI,
    pub ema: EMA,
    pub stock_rsi: StochRSI,
    pub adx: ADX,
    pub macd: MACD,
    pub tr_rma: TrRMA,
    pub tr: TR,
    pub pre_tr_rma: f64,
}

impl IndicatorsCollection {
    pub fn new() -> Self {
        Self {
            dc: DC::new(None),
            rsi: RSI::new(()),
            ema: EMA::new(None),
            stock_rsi: StochRSI::new(None),
            adx: ADX::new(()),
            macd: MACD::new(()),
            tr_rma: TrRMA::new(),
            tr: TR::new(()),
            pre_tr_rma: 0.,
        }
    }
}
