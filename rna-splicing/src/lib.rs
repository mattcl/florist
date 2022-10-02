use std::borrow::Cow;

use florist_core::{DNASequence, ProteinSequence};
use florist_fasta::OrderedMultiFasta;
use florist_plumbing::Problem;

pub struct RnaSplicing;

impl Problem for RnaSplicing {
    type Error = anyhow::Error;
    type Input = OrderedMultiFasta;
    type Output = ProteinSequence;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut values = input.iter().map(|(_, v)| v);
        // This unwrap is safe because MultiFasta guarantees at least one.
        let mut raw_seq: Cow<str> = values.next().unwrap().into();

        for intron in values {
            raw_seq = raw_seq.replace(intron, "XXX").into();
        }

        let seq: DNASequence = raw_seq.replace("XXX", "").parse()?;

        Ok(seq.try_into()?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn rns_splicing() {
        let raw = ">Rosalind_10
ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG
>Rosalind_12
ATCGGTCGAA
>Rosalind_15
ATCGGTCGAGCGTGT";
        let input = OrderedMultiFasta::from_str(raw).unwrap();
        let output = RnaSplicing::solve(input).unwrap();
        assert_eq!(
            output,
            ProteinSequence::from_str("MVYIADKQHVASREAYGHMFKVCA").unwrap()
        );
    }
}
