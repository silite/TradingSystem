use std::borrow::Cow;

/// Clone only for Builder.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Exchange(Cow<'static, str>);

impl<E> From<E> for Exchange
where
    E: Into<Cow<'static, str>>,
{
    fn from(exchange: E) -> Self {
        Exchange(exchange.into())
    }
}

impl Exchange {
    pub fn name(&self) -> &str {
        &self.0
    }
}
