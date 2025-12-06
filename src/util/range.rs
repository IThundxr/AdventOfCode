use crate::error::Error;

/// A simple `x-y` range, for example: `3-10`
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    pub fn parse(s: &str) -> Result<Self, Error> {
        let Some((start, end)) = s.split_once("-") else {
            return Err(Error::ParseRangeError(s.into()));
        };
        let start = start.parse::<usize>().map_err(Error::ParseIntError)?;
        let end = end.parse::<usize>().map_err(Error::ParseIntError)?;

        Ok(Self::new(start, end))
    }

    pub fn contains(&self, i: usize) -> bool {
        self.start <= i && self.end >= i
    }

    /// Get the integers contained in this range
    pub fn get_integers(&self) -> Vec<usize> {
        Vec::from_iter(self.start..self.end + 1)
    }
}
