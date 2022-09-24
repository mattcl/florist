use std::str::FromStr;

use anyhow::bail;
use florist_core::{DNASequence, Error, HammingDistance};
use florist_plumbing::Problem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SequencePair {
    first: DNASequence,
    second: DNASequence,
}

impl SequencePair {
    fn hamming_distance(&self) -> Result<u64, Error> {
        self.first.hamming_distance(&self.second)
    }
}

impl FromStr for SequencePair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split("\n").collect();

        if parts.len() != 2 {
            bail!("Invalid input, wrong number of sequences: {}", s);
        }

        Ok(SequencePair {
            first: parts[0].trim().parse()?,
            second: parts[1].trim().parse()?,
        })
    }
}

pub struct CountingPointMutations;

impl Problem for CountingPointMutations {
    type Error = Error;
    type Input = SequencePair;
    type Output = u64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.hamming_distance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_mutations() {
        let raw = "
GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT
";
        let input = SequencePair::from_str(raw).unwrap();
        let output = CountingPointMutations::solve(input).unwrap();
        assert_eq!(output, 7);
    }
}
