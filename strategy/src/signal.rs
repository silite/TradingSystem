pub trait SignalGenerator {
    /// Optionally return a [`Signal`];
    // TODO
    fn generate_signal(&mut self) -> Option<impl SignalExt>;
}

pub trait SignalExt {}
