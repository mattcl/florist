use florist_core::{DNASequence, Error};
use florist_plumbing::Problem;

pub struct ComplementingAStrandOfDna;

impl Problem for ComplementingAStrandOfDna {
    type Error = Error;
    type Input = DNASequence;
    type Output = DNASequence;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.reverse_complement())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn complement() {
        let input = DNASequence::from_str("AAAACCCGGT").expect("Failed to parse");
        let output = ComplementingAStrandOfDna::solve(input).expect("Failed to solve");
        assert_eq!(output, DNASequence::from_str("ACCGGGTTTT").unwrap());
    }
}
