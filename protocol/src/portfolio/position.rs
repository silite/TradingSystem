use crate::{
    market::{exchange::Exchange, symbol::Instrument, Market},
    trade::{fee::Fees, Side},
};

#[derive(Clone)]
pub struct MetaPosition {
    /// Metadata detailing trace UUIDs, timestamps & equity associated with entering, updating & exiting.
    // pub meta: PositionMeta,

    /// Buy or Sell.
    ///
    /// Notes:
    /// - Side::Buy considered synonymous with Long.
    /// - Side::Sell considered synonymous with Short.
    pub side: Side,

    /// +ve or -ve quantity of symbol contracts opened.
    pub quantity: f64,

    /// All fees types incurred from entering a [`Position`], and their associated [`FeeAmount`].
    pub enter_fees: Fees,

    /// Total of enter_fees incurred. Sum of every [`FeeAmount`] in [`Fees`] when entering a [`Position`].
    pub enter_fees_total: f64,

    /// Enter average price excluding the entry_fees_total.
    pub enter_avg_price_gross: f64,

    /// abs(Quantity) * enter_avg_price_gross.
    pub enter_value_gross: f64,

    /// All fees types incurred from exiting a [`Position`], and their associated [`FeeAmount`].
    pub exit_fees: Fees,

    /// Total of exit_fees incurred. Sum of every [`FeeAmount`] in [`Fees`] when entering a [`Position`].
    pub exit_fees_total: f64,

    /// Exit average price excluding the exit_fees_total.
    pub exit_avg_price_gross: f64,

    /// abs(Quantity) * exit_avg_price_gross.
    pub exit_value_gross: f64,

    /// Symbol current close price.
    pub current_symbol_price: f64,

    /// abs(Quantity) * current_symbol_price.
    pub current_value_gross: f64,

    /// Unrealized P&L whilst the [`Position`] is open.
    pub unrealized_profit_loss: f64,

    /// Realized P&L after the [`Position`] has closed.
    pub realized_profit_loss: f64,
}
