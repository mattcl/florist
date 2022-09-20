use std::fmt::Display;

use florist_core::{DNASequence, Error};
use florist_plumbing::Problem;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct NucleotideCounts {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}

impl Display for NucleotideCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.c, self.g, self.t)
    }
}

pub struct CountingDnaNucleotides;

impl Problem for CountingDnaNucleotides {
    type Error = Error;
    type Input = DNASequence;
    type Output = NucleotideCounts;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut counts = NucleotideCounts::default();
        input.chars().for_each(|ch| match ch {
            'A' => counts.a += 1,
            'C' => counts.c += 1,
            'G' => counts.g += 1,
            'T' => counts.t += 1,
            _ => unreachable!(),
        });

        Ok(counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn counting() {
        let input = DNASequence::from_str(
            "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC",
        )
        .expect("Invalid input");

        let output = CountingDnaNucleotides::solve(input).expect("Failed to solve");

        assert_eq!(
            output,
            NucleotideCounts {
                a: 20,
                c: 12,
                g: 17,
                t: 21,
            }
        )
    }
}
