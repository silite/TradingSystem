use core::error;

use protocol::portfolio::amount::Amount;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortfolioError {
    #[error("可用资金不足, 可用{0}, 需要{1}.")]
    OpenBalanceInsufficient(Amount, Amount),
    #[error("冻结资金不足, 可用{0}, 需要{1}.")]
    FreezedBalanceInsufficient(Amount, Amount),
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for PortfolioError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(error.to_string())
    }
}
