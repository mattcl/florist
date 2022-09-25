use florist_core::{sequence::ProteinSequence, Error, RNASequence};
use florist_plumbing::Problem;

pub struct TranslatingRnaIntoProtein;

impl Problem for TranslatingRnaIntoProtein {
    type Error = Error;
    type Input = RNASequence;
    type Output = ProteinSequence;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.try_into()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn rna_to_protein() {
        let raw = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
        let input = RNASequence::from_str(raw).unwrap();
        let output = TranslatingRnaIntoProtein::solve(input).unwrap();
        assert_eq!(
            output,
            ProteinSequence::from_str("MAMAPRTEINSTRING").unwrap()
        );
    }
}
