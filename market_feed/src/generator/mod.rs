use crate::MarketFeed;

pub mod binance;

pub enum Feed<Event> {
    Next(Event),
    Unhealthy,
    Finished,
}

/// Generates the next `Event`. Acts as the system heartbeat.
pub trait MarketGenerator<Event>: Sized {
    fn new(rx: crossbeam::channel::Receiver<Event>) -> Self;

    /// Return the next market `Event`.
    fn next(&mut self) -> Feed<Event>;
}
