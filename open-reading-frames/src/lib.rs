use std::{collections::HashSet, fmt::Display};

use florist_core::{DNASequence, Error, ProteinSequence};
use florist_fasta::MultiFasta;
use florist_plumbing::Problem;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Output(HashSet<ProteinSequence>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: Vec<_> = self.0.iter().map(|s| s.to_string()).collect();
        output.sort();
        output.join("\n").fmt(f)
    }
}

pub struct OpenReadingFrames;

impl Problem for OpenReadingFrames {
    type Error = Error;
    type Input = MultiFasta;
    type Output = Output;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut values = input.values();
        let mut output = HashSet::new();
        // This unwrap is safe because MultiFasta guarantees at least one value.
        let seq: DNASequence = values.next().unwrap().parse()?;

        for frame in seq.open_frames() {
            if let Ok(protein) = ProteinSequence::try_from(frame) {
                output.insert(protein);
            }
        }

        let comp = seq.reverse_complement();
        for frame in comp.open_frames() {
            if let Ok(protein) = ProteinSequence::try_from(frame) {
                output.insert(protein);
            }
        }

        Ok(Output(output))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn reading_frames() {
        let raw = ">Rosalind_99
AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG";
        let input = MultiFasta::from_str(raw).unwrap();
        let output = OpenReadingFrames::solve(input).unwrap();
        let expected = "M
MGMTPRLGLESLLE
MLLGSFRLIPKETLIQVAGSSPCNLS
MTPRLGLESLLE";

        assert_eq!(output.to_string(), expected.to_string());
    }
}
