#[derive(Clone)]
pub struct Fees {
    /// Fee taken by the exchange/broker (eg/ commission).
    pub exchange: f64,
    /// Order book slippage modelled as a fee.
    pub slippage: f64,
    /// Fee incurred by any required network transactions (eg/ GAS).
    pub network: f64,
}
