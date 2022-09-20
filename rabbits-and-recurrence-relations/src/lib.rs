use std::{num::ParseIntError, str::FromStr};

use florist_plumbing::Problem;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Input has wrong number of values: {0}")]
    WrongNumberOfValues(String),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Recurrence {
    months: usize,
    litter_size: usize,
}

impl FromStr for Recurrence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<_> = s
            .split_whitespace()
            .map(usize::from_str)
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        if params.len() != 2 {
            return Err(Error::WrongNumberOfValues(s.into()));
        }

        Ok(Self {
            months: params[0],
            litter_size: params[1],
        })
    }
}

pub struct RabbitsAndRecurrenceRelations;

impl Problem for RabbitsAndRecurrenceRelations {
    type Error = Error;
    type Input = Recurrence;
    type Output = usize;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut prev = 1_usize;
        let mut cur = 1_usize;

        for _ in 2..input.months {
            let next = prev * input.litter_size + cur;
            prev = cur;
            cur = next;
        }

        Ok(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn growth() {
        let input = Recurrence::from_str("5 3").unwrap();
        let output = RabbitsAndRecurrenceRelations::solve(input).unwrap();
        assert_eq!(output, 19);
    }
}
