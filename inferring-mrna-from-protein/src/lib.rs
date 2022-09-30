use florist_core::{AminoAcid, Error, ProteinSequence};
use florist_plumbing::Problem;

pub struct InferringMrnaFromProtein;

impl Problem for InferringMrnaFromProtein {
    type Error = Error;
    type Input = ProteinSequence;
    type Output = u64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut working = 1_u64;
        let mut running = 1_u64;

        for ch in input.chars() {
            if working >= 100_000_000 {
                running *= working % 1_000_000;
                running %= 1_000_000;
                working = 1;
            }

            let count = AminoAcid::try_from(ch)?.num_codons() as u64;
            working *= count;
        }

        Ok((running * working % 1_000_000 * 3) % 1_000_000)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn mod_count() {
        let input = ProteinSequence::from_str("MA").unwrap();
        let output = InferringMrnaFromProtein::solve(input).unwrap();
        assert_eq!(output, 12);
    }
}
