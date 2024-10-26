use std::ops::Not;

pub mod fee;

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        if matches!(self, Self::Buy) {
            return Self::Sell;
        }
        Self::Buy
    }
}
