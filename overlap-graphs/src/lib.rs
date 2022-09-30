use std::{fmt::Display, ops::Deref};

use florist_core::Error;
use florist_fasta::MultiFasta;
use florist_plumbing::Problem;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Graph(Vec<(String, String)>);

impl Graph {
    pub fn edge(&mut self, left: String, right: String) {
        self.0.push((left, right));
    }

    pub fn sort(&mut self) {
        self.0.sort()
    }
}

impl Deref for Graph {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .map(|(left, right)| format!("{} {}", left, right))
            .collect::<Vec<_>>()
            .join("\n")
            .fmt(f)
    }
}

pub struct OverlapGraphs;

impl Problem for OverlapGraphs {
    type Error = Error;
    type Input = MultiFasta;
    type Output = Graph;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut graph = Graph::default();

        for (desc, seq) in input.iter() {
            for (other, other_seq) in input.iter() {
                if desc == other {
                    continue;
                }

                if seq.len() < 3 || other.len() < 3 {
                    continue;
                }

                if let (Some(pref), Some(suff)) = (seq.get((seq.len() - 3)..), other_seq.get(0..3))
                {
                    if pref == suff {
                        graph.edge(desc.to_string(), other.to_string())
                    }
                }
            }
        }

        Ok(graph)
    }
}

pub struct SortedOverlapGraphs;

impl Problem for SortedOverlapGraphs {
    type Error = Error;
    type Input = MultiFasta;
    type Output = Graph;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut graph = Graph::default();

        for (desc, seq) in input.iter() {
            for (other, other_seq) in input.iter() {
                if desc == other {
                    continue;
                }

                if seq.len() < 3 || other.len() < 3 {
                    continue;
                }

                if let (Some(pref), Some(suff)) = (seq.get((seq.len() - 3)..), other_seq.get(0..3))
                {
                    if pref == suff {
                        graph.edge(desc.to_string(), other.to_string())
                    }
                }
            }
        }

        // this is going to cost us time, but I guess it'll make the test
        // reliable
        graph.sort();

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn overlaps() {
        let raw = ">Rosalind_0498
AAATAAA
>Rosalind_2391
AAATTTT
>Rosalind_2323
TTTTCCC
>Rosalind_0442
AAATCCC
>Rosalind_5013
GGGTGGG";
        let input = MultiFasta::from_str(raw).unwrap();
        let mut output = OverlapGraphs::solve(input).unwrap();
        output.sort();

        let mut expected = Graph::default();
        expected.edge("Rosalind_0498".to_string(), "Rosalind_2391".to_string());
        expected.edge("Rosalind_0498".to_string(), "Rosalind_0442".to_string());
        expected.edge("Rosalind_2391".to_string(), "Rosalind_2323".to_string());
        expected.sort();

        assert_eq!(output, expected);
    }
}
