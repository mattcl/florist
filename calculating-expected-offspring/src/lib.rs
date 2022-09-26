use std::ops::Deref;

use florist_core::{Error, SingleGenePopulation};
use florist_inputs::U32List;
use florist_plumbing::Problem;

pub struct CalculatingExpectedOffspring;

impl Problem for CalculatingExpectedOffspring {
    type Error = Error;
    type Input = U32List;
    type Output = f64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let pop = SingleGenePopulation::try_from(input.deref())?;
        Ok(pop.expected_dominant_offspring(2))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn expected_offspring() {
        let raw = "1 0 0 1 0 1";
        let input = U32List::from_str(raw).unwrap();
        let output = CalculatingExpectedOffspring::solve(input).unwrap();
        assert_eq!(output, 3.5);
    }
}
