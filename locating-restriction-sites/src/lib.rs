use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use florist_core::DNASequence;
use florist_fasta::MultiFasta;
use florist_plumbing::Problem;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Output(Vec<(usize, usize)>);

impl Deref for Output {
    type Target = Vec<(usize, usize)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Output {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|(idx, size)| format!("{} {}", idx, size))
            .collect::<Vec<_>>()
            .join("\n")
            .fmt(f)
    }
}

pub struct LocatingRestrictionSites;

impl Problem for LocatingRestrictionSites {
    type Error = anyhow::Error;
    type Input = MultiFasta;
    type Output = Output;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        // we know the input has to have at least one entry for it to have been
        // parsed, so the unwrap is safe.
        let seq = DNASequence::from_str(input.values().next().unwrap())?;
        let len = seq.len();
        let comp = seq.reverse_complement();
        let seq_bytes = seq.as_bytes();
        let comp_bytes = comp.as_bytes();

        let points: Vec<(usize, usize)> = (4..=12)
            .flat_map(|window| {
                (0..(len - window + 1))
                    .filter_map(|idx| {
                        let comp_start = len - window - idx;
                        let comp_end = len - idx;
                        if seq_bytes[idx..(idx + window)] == comp_bytes[comp_start..comp_end] {
                            // seriously, who indexes things starting at 1
                            Some((idx + 1, window))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Output(points))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locating_sites() {
        let raw = ">Rosalind_24
TCAATGCATGCGGGTCTATATGCAT";
        let input = MultiFasta::from_str(raw).unwrap();
        let mut output = LocatingRestrictionSites::solve(input).unwrap();
        output.sort();

        let mut expected = Output(vec![
            (4, 6),
            (5, 4),
            (6, 6),
            (7, 4),
            (17, 4),
            (18, 4),
            (20, 6),
            (21, 4),
        ]);
        expected.sort();

        assert_eq!(output, expected);
    }
}
