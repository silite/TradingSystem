use std::borrow::Cow;

use super::InstrumentKind;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Instrument {
    pub base: Cow<'static, str>,
    pub quote: Cow<'static, str>,
    pub kind: InstrumentKind,
}

impl<S> From<(S, S, InstrumentKind)> for Instrument
where
    S: Into<Cow<'static, str>>,
{
    fn from(value: (S, S, InstrumentKind)) -> Self {
        Self {
            base: value.0.into(),
            quote: value.1.into(),
            kind: value.2,
        }
    }
}

impl Instrument {
    pub fn symbol(&self) -> String {
        format!("{}{}", self.base, self.quote)
    }
}
