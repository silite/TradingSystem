#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod balance;
pub mod error;
pub mod position;

use balance::BalanceHandler;
use derive_builder::Builder;
use error::PortfolioError;
use position::PositionHandler;
use protocol::{
    market::{symbol::Instrument, Market},
    portfolio::{amount::Amount, position::MetaPosition},
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

#[derive(Builder, Clone)]
pub struct MetaPortfolio {
    /// 上层EngineID
    engine_id: Uuid,
    /// 市场标的，一个Trader对应一个Portfolio
    market: Market,
    /// 可用资金
    open_balance: Amount,
    /// 冻结资金
    freezed_balance: Amount,
    /// 退出交易资金
    exited_balance: Amount,
    /// 可用仓位
    open_position: Vec<MetaPosition>,
    /// 冻结仓位
    freezed_position: Vec<MetaPosition>,
    /// 退出交易仓位
    exited_position: Vec<MetaPosition>,
    // 最后更新时间，暂时无用
    // update_ms: u64,
}

impl BalanceHandler for MetaPortfolio {
    fn set_available_balance<A: Into<Amount>>(
        &mut self,
        diff: A,
    ) -> anyhow::Result<(), PortfolioError> {
        self.open_balance = diff.into();
        Ok(())
    }

    fn diff_available_balance<A: Into<Amount>>(
        &mut self,
        diff: A,
    ) -> anyhow::Result<(), PortfolioError> {
        let diff = diff.into();
        if diff < dec!(0.).into() && &self.open_balance < &-diff {
            return Err(PortfolioError::OpenBalanceInsufficient(
                self.open_balance,
                -diff,
            ));
        }

        self.open_balance += diff;

        Ok(())
    }

    fn get_available_balance(&self) -> Amount {
        self.open_balance
    }

    fn diff_open_freezed_balance<A: Into<Amount>>(
        &mut self,
        diff: A,
    ) -> anyhow::Result<(), PortfolioError> {
        let diff = diff.into();
        if diff < dec!(0.).into() && &self.freezed_balance < &-diff {
            return Err(PortfolioError::FreezedBalanceInsufficient(
                self.freezed_balance,
                -diff,
            ));
        } else if diff > dec!(0.).into() && &self.open_balance < &diff {
            return Err(PortfolioError::OpenBalanceInsufficient(
                self.open_balance,
                diff,
            ));
        }

        self.open_balance -= diff;
        self.freezed_balance += diff;

        Ok(())
    }

    fn set_freezed_balance<A: Into<Amount>>(
        &mut self,
        diff: A,
    ) -> anyhow::Result<(), PortfolioError> {
        self.freezed_balance = diff.into();

        Ok(())
    }

    fn diff_freezed_balance<A: Into<Amount>>(
        &mut self,
        diff: A,
    ) -> anyhow::Result<(), PortfolioError> {
        let diff = diff.into();
        ftlog::info!("[diff_freezed_balance] {} {}", self.freezed_balance, diff);
        if diff < dec!(0.).into() && &self.freezed_balance < &-diff {
            return Err(PortfolioError::FreezedBalanceInsufficient(
                self.freezed_balance,
                -diff,
            ));
        }

        self.freezed_balance += diff;

        Ok(())
    }

    fn get_freezed_balance(&self) -> Amount {
        self.freezed_balance
    }

    fn set_exit_balance<A: Into<Amount>>(&mut self, diff: A) -> anyhow::Result<(), PortfolioError> {
        todo!()
    }

    fn diff_exit_balance<A: Into<Amount>>(
        &mut self,
        diff: A,
    ) -> anyhow::Result<(), PortfolioError> {
        todo!()
    }

    fn get_exit_balance(&self) -> Amount {
        self.exited_balance
    }
}

impl PositionHandler for MetaPortfolio {
    fn position_id(&self) -> &protocol::market::Market {
        todo!()
    }

    fn set_open_position(
        &mut self,
        position: protocol::portfolio::position::MetaPosition,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn diff_open_position(
        &mut self,
        position: protocol::portfolio::position::MetaPosition,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn get_open_positions(
        &self,
    ) -> anyhow::Result<&Vec<protocol::portfolio::position::MetaPosition>> {
        todo!()
    }

    fn set_freezed_position(
        &mut self,
        position: protocol::portfolio::position::MetaPosition,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn diff_freezed_position(
        &mut self,
        position: protocol::portfolio::position::MetaPosition,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn get_freezed_positions(
        &self,
    ) -> anyhow::Result<&Vec<protocol::portfolio::position::MetaPosition>> {
        todo!()
    }

    fn remove_position(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    fn set_exited_position(
        &mut self,
        position: protocol::portfolio::position::MetaPosition,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn diff_exited_position(
        &mut self,
        position: protocol::portfolio::position::MetaPosition,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn get_exited_positions(
        &self,
    ) -> anyhow::Result<&Vec<protocol::portfolio::position::MetaPosition>> {
        todo!()
    }
}
