use florist_core::{sequence::RNASequence, DNASequence, Error};
use florist_plumbing::Problem;

pub struct TranscribingDnaIntoRna;

impl Problem for TranscribingDnaIntoRna {
    type Error = Error;
    type Input = DNASequence;
    type Output = RNASequence;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn conversion() {
        let input = DNASequence::from_str("GATGGAACTTGACTACGTAAATT").expect("Failed to parse");
        let output = TranscribingDnaIntoRna::solve(input).expect("Failed to solve");
        assert_eq!(
            output,
            RNASequence::from_str("GAUGGAACUUGACUACGUAAAUU").unwrap()
        );
    }
}
