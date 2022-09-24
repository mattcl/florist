use std::{fmt::Display, str::FromStr};

use anyhow::{Result, anyhow};
use florist_core::{DNASequence, GCContent, Error};
use florist_fasta::MultiFasta;
use florist_plumbing::Problem;


#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    description: String,
    gc_content: f64,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.description, self.gc_content * 100.0)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ComputingGcContent;

impl Problem for ComputingGcContent {
    type Error = anyhow::Error;
    type Input = MultiFasta;
    type Output = Output;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let content = input
            .iter()
            .map(|(desc, seq)| DNASequence::from_str(seq).map(|parsed| (desc, parsed.gc_content())))
            .collect::<Result<Vec<(&String, f64)>, Error>>()?;

        let (desc, gc) = content
            .iter()
            // calling the case where we can't get a partial comparison equal is probably wrong
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or_else(|| anyhow!("No sequences"))?;

        Ok(Output { description: desc.to_string(), gc_content: *gc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gc_content() {
        let data = "
>Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT
";
        let input = MultiFasta::from_str(data).unwrap();
        let output = ComputingGcContent::solve(input).unwrap();

        assert_eq!(output.description, "Rosalind_0808".to_string());
        assert!(output.gc_content - 0.6091954022988506 <= f64::EPSILON);
    }
}
