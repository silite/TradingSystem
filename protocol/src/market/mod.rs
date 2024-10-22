use std::hash::Hash;

use exchange::Exchange;
use symbol::Instrument;

pub mod exchange;
pub mod symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum InstrumentKind {
    Spot,
    // TODO expiry
    Future,
    Perpetual,
    // TODO kind、exercise、expiry、strike
    Option,
}

/// Clone only for Builder.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Market<InstrumentId = Instrument>
where
    InstrumentId: Clone,
{
    pub exchange: Exchange,
    pub instrument: InstrumentId,
}

impl Market<Instrument> {
    /// Constructs a new [`Market`] using the provided [`Exchange`] & [`Instrument`].
    pub fn new<E, I>(exchange: E, instrument: I) -> Self
    where
        E: Into<Exchange>,
        I: Into<Instrument>,
    {
        Self {
            exchange: exchange.into(),
            instrument: instrument.into(),
        }
    }
}
