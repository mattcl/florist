pub mod sequence;

pub use sequence::{DNASequence, GCContent, HammingDistance, RNASequence};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid character in DNA sequence: {0}")]
    InvalidDNASequence(char),

    #[error("Invalid character in RNA sequence: {0}")]
    InvalidRNASequence(char),

    #[error("Empty strings are not valid sequences")]
    EmptySequence,

    #[error("Sequences must be equal in length")]
    NotEqualLength,
}
