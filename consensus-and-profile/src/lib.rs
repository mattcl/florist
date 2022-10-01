use florist_core::{Consensus, DNASequence, Error};
use florist_fasta::MultiFasta;
use florist_plumbing::Problem;

pub struct ConsensusAndProfile;

impl Problem for ConsensusAndProfile {
    type Error = Error;
    type Input = MultiFasta;
    type Output = Consensus<DNASequence>;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let seqs = input
            .values()
            .cloned()
            .map(DNASequence::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Consensus::try_from_iter(seqs.iter())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn consensus() {
        let raw = ">Rosalind_1
ATCCAGCT
>Rosalind_2
GGGCAACT
>Rosalind_3
ATGGATCT
>Rosalind_4
AAGCAACC
>Rosalind_5
TTGGAACT
>Rosalind_6
ATGCCATT
>Rosalind_7
ATGGCACT";
        let input = MultiFasta::from_str(raw).unwrap();
        let output = ConsensusAndProfile::solve(input).unwrap();
        let expected = "ATGCAACT
A: 5 1 0 0 5 5 0 0
C: 0 0 1 4 2 0 6 1
G: 1 1 6 3 0 1 0 0
T: 1 5 0 0 0 1 1 6"
            .to_string();

        assert_eq!(output.to_string(), expected);
    }
}
