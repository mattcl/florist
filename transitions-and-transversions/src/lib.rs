use anyhow::bail;
use florist_core::{DNASequence, Substitutable};
use florist_fasta::MultiFasta;
use florist_plumbing::Problem;

pub struct TransitionsAndTransversions;

impl Problem for TransitionsAndTransversions {
    type Error = anyhow::Error;
    type Input = MultiFasta;
    type Output = f64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.len() != 2 {
            bail!("Wrong number of sequences in input");
        }

        let sequences: Vec<DNASequence> = input
            .values()
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;

        Ok(sequences[0].transition_transversion_ratio(&sequences[1])?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn finding_ratio() {
        let raw = ">Rosalind_0209
GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGTGATCGTTGTAGTTATTGGA
AGTACGGGCATCAACCCAGTT
>Rosalind_2200
TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGCGATCGTGCTGGTTACTGGC
GGTACGAGTGTTCCTTTGGGT";
        let input = MultiFasta::from_str(raw).unwrap();
        let output = TransitionsAndTransversions::solve(input).unwrap();
        assert!((output - 1.2142857142857142).abs() < f64::EPSILON);
    }
}
