use std::{
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Default, Debug, Copy)]
pub struct Volume(pub f64);

impl Add for Volume {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Volume {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl SubAssign for Volume {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (self.0 - rhs.0).into();
    }
}

impl AddAssign for Volume {
    fn add_assign(&mut self, rhs: Self) {
        *self = (self.0 + rhs.0).into();
    }
}

impl Neg for Volume {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<f64> for Volume {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
