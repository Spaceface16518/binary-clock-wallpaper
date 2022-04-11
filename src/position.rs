use std::{num::ParseIntError, ops::Index, str::FromStr};

use memchr::memchr;
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Positions<const N: usize>(#[serde_as(as = "[_; N]")] [i64; N]);

impl<const N: usize> Index<usize> for Positions<N> {
    type Output = <[i64; N] as Index<usize>>::Output;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> Positions<N> {
    pub fn resize<const M: usize>(&self) -> Positions<M> {
        let mut new = [0; M];
        new.copy_from_slice(&self.0[..M]);
        Positions(new)
    }

    pub fn single(&self, i: usize) -> Positions<1> {
        Positions([self.0[i]])
    }

    pub fn iter(&self) -> std::slice::Iter<'_, i64> {
        self.0.iter()
    }
}

pub type Hour = Positions<5>;

pub type Minute = Positions<6>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum PositionParseError {
    #[error("error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("incorrect amount of arguments: {0}")]
    AmountError(usize),
}

impl<const N: usize> FromStr for Positions<N> {
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: don't allocate here
        let amounts = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Positions(amounts.as_slice().try_into().map_err(|_| {
            PositionParseError::AmountError(amounts.len())
        })?))
    }
}

/// Wraps `Hour` or `Minute` to allow user to enter either a singular value or a
/// value for each x-axis in a type-safe way.
#[derive(Clone, Debug, Deserialize)]
pub enum YAxis<Time> {
    /// A singluar y-axis position repeated for each x-axis position
    Singular(i64),
    /// Each x-axis position has its own y-axis position
    Variable(Time),
}

impl<const N: usize> Index<usize> for YAxis<Positions<N>> {
    type Output = <Positions<N> as Index<usize>>::Output;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            YAxis::Singular(i) => i,
            YAxis::Variable(pos) => pos.index(index),
        }
    }
}

impl<const N: usize> YAxis<Positions<N>> {
    pub fn resize<const M: usize>(&self) -> YAxis<Positions<M>> {
        match self {
            YAxis::Variable(positions) => YAxis::Variable(positions.resize::<M>()),
            YAxis::Singular(i) => YAxis::Singular(*i),
        }
    }

    pub fn single(&self, i: usize) -> YAxis<Positions<1>> {
        match self {
            YAxis::Variable(positions) => YAxis::Variable(positions.single(i)),
            YAxis::Singular(i) => YAxis::Singular(*i),
        }
    }
}

impl<Time> FromStr for YAxis<Time>
where
    Time: FromStr<Err = PositionParseError>,
{
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if it contains a comma, we'll treat it as variable
        if memchr(b',', s.as_bytes()).is_some() {
            Ok(YAxis::Variable(s.parse()?))
        } else {
            Ok(YAxis::Singular(s.parse()?))
        }
    }
}
