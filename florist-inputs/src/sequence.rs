use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use florist_core::{DNASequence, ProteinSequence, RNASequence, Sequence};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct SequenceList<T>(Vec<T>)
where
    T: Sequence;

impl<T: Sequence> Deref for SequenceList<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Sequence> DerefMut for SequenceList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Sequence> FromStr for SequenceList<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(sequences: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            sequences
                .trim()
                .split("\n")
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<T>, _>>()?,
        ))
    }
}

impl<T: Sequence> Display for SequenceList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        out.fmt(f)
    }
}

// convenience wrappers

/// A list of DNA sequences built from a newline-separated input string
pub type DNASequenceList = SequenceList<DNASequence>;

/// A list of RNA sequences built from a newline-separated input string
pub type RNASequenceList = SequenceList<RNASequence>;

/// A list of protein sequences built from a newline-separated input string
pub type ProteinSequenceList = SequenceList<ProteinSequence>;
