use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Amount(f64);

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<f64> for Amount {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

pub trait BalanceHandler {
    /// 设置资金信息。
    fn set_available_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<()>;

    /// diff当前可用资金，-为流出，+为流入。
    fn diff_available_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<()>;

    /// 获取可用资金信息。
    fn get_available_balance(&self) -> Amount;

    /// 设置冻结资金信息。
    fn set_freezed_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<()>;

    /// diff当前冻结资金，-为流出，+为流入。
    fn diff_freezed_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<()>;

    /// 获取在途、冻结、挂单中的资金。
    fn get_freezed_balance(&self) -> Amount;

    /// 设置退出交易的资金。
    fn set_exit_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<()>;

    /// diff退出交易的资金，-为转回交易，+为退出交易。
    fn diff_exit_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<()>;

    /// 获取退出交易的资金。
    fn get_exit_balance(&self) -> Amount;

    fn get_total_balance(&self) -> Amount {
        self.get_available_balance() + self.get_freezed_balance() + self.get_exit_balance()
    }
}
