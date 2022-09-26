use std::fmt::Display;

use anyhow::bail;
use florist_core::Motif;
use florist_inputs::DNASequenceList;
use florist_plumbing::Problem;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct MotifLocations(Vec<usize>);

impl From<Vec<usize>> for MotifLocations {
    fn from(v: Vec<usize>) -> Self {
        Self(v)
    }
}

impl Display for MotifLocations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ").fmt(f)
    }
}

pub struct FindingAMotifInDna;

impl Problem for FindingAMotifInDna {
    type Error = anyhow::Error;
    type Input = DNASequenceList;
    type Output = MotifLocations;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.len() != 2 {
            bail!("Input is not the right number (2) of sequences");
        }

        Ok(
            input[0]
                .motif_lcoations(&input[1])
                .iter()
                .map(|v| v + 1)
                .collect::<Vec<_>>()
                .into()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn finding_motif() {
        let raw = "GATATATGCATATACTT
ATAT";

        let input = DNASequenceList::from_str(raw).unwrap();
        let output = FindingAMotifInDna::solve(input).unwrap();
        assert_eq!(output.to_string(), "2 4 10")
    }

}
