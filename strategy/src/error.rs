use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreValidError {}

#[derive(Error, Debug)]
pub enum RearValidError {}

#[derive(Error, Debug)]
pub enum OpenError {
    #[error("EMA不符合条件, EMA({0:.2}), Low({1:.2}), High({2:.2})")]
    EMAIncompatible(f64, f64, f64),
    #[error("MACD不符合条件, MACD({0:.2}, {1:.2}), Low({2:.2}), High({3:.2})")]
    MACDIncompatible(f64, f64, f64, f64),
    #[error("ADX不足, ADX({0:.2})")]
    ADXInsufficient(f64),
    #[error("StochRSI没在超卖区形成金叉, k({0:.2}), d({1:.2})")]
    StochRSIOversoldIncompatible(f64, f64),
    #[error("StochRSI没在超买区形成死叉, k({0:.2}), d({1:.2})")]
    StochRSIOverboughtIncompatible(f64, f64),
    #[error("StochRSI趋势不足")]
    StochRSIInsufficient,
    #[error("连续开单时间过短")]
    OpenQuickSuccession,
    #[error("MACD呈衰弱")]
    MACDInsufficient,
    #[error("可用资金不足")]
    AvailableFundInsufficient,
    #[error("构建订单失败: {0}")]
    BuildOrderError(String),
}

impl From<anyhow::Error> for OpenError {
    fn from(err: anyhow::Error) -> Self {
        OpenError::BuildOrderError(err.to_string())
    }
}

#[derive(Error, Debug)]
pub enum CloseError {
    #[error("todo")]
    Test,
}
