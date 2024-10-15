#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod balance;
pub mod position;

use balance::{Amount, BalanceHandler};
use derive_builder::Builder;
use position::PositionHandler;
use protocol::{
    market::{symbol::Instrument, Market},
    portfolio::position::MetaPosition,
};
use uuid::Uuid;

#[derive(Builder, Clone)]
pub struct MetaPortfolio {
    ///
    engine_id: Uuid,
    ///
    market: Market,
    ///
    open_balance: Amount,
    ///
    freezed_balance: Amount,
    ///
    exited_balance: Amount,
    ///
    open_position: Vec<MetaPosition>,
    ///
    freezed_position: Vec<MetaPosition>,
    ///
    exited_position: Vec<MetaPosition>,
    ///
    update_ms: u64,
}

impl BalanceHandler for MetaPortfolio {
    fn set_available_balance<A: Into<balance::Amount>>(&mut self, diff: A) -> anyhow::Result<()> {
        todo!()
    }

    fn diff_available_balance<A: Into<balance::Amount>>(&mut self, diff: A) -> anyhow::Result<()> {
        todo!()
    }

    fn get_available_balance(&self) -> balance::Amount {
        todo!()
    }

    fn set_freezed_balance<A: Into<balance::Amount>>(&mut self, diff: A) -> anyhow::Result<()> {
        todo!()
    }

    fn diff_freezed_balance<A: Into<balance::Amount>>(&mut self, diff: A) -> anyhow::Result<()> {
        todo!()
    }

    fn get_freezed_balance(&self) -> balance::Amount {
        todo!()
    }

    fn set_exit_balance<A: Into<balance::Amount>>(&mut self, diff: A) -> anyhow::Result<()> {
        todo!()
    }

    fn diff_exit_balance<A: Into<balance::Amount>>(&mut self, diff: A) -> anyhow::Result<()> {
        todo!()
    }

    fn get_exit_balance(&self) -> balance::Amount {
        todo!()
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
