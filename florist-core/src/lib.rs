pub mod amino;
pub mod codon;
pub mod sequence;

pub use amino::AminoAcid;
pub use codon::{DNACodon, RNACodon};
pub use sequence::{
    Sequence,
    DNASequence,
    GCContent,
    GeneticSequence,
    HammingDistance,
    RNASequence,
    ProteinSequence,
    Motif,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid character in DNA sequence: {0}")]
    InvalidDNASequence(char),

    #[error("Invalid character in RNA sequence: {0}")]
    InvalidRNASequence(char),

    #[error("Invalid character in protein sequence: {0}")]
    InvalidProteinSequence(char),

    #[error("Empty strings are not valid sequences")]
    EmptySequence,

    #[error("Sequences must be equal in length")]
    NotEqualLength,

    #[error("Codon input is wrong length: {0}")]
    CodonWrongLength(String),

    #[error("DNA Codon is unknown: {0}")]
    UnknownDnaCodon(DNACodon),

    #[error("RNA Codon is unknown: {0}")]
    UnknownRnaCodon(RNACodon),

    #[error("Invalid character in DNA codon: {0}")]
    InvalidDNACodon(char),

    #[error("Invalid character in RNA codon: {0}")]
    InvalidRNACodon(char),

    #[error("Cannot reconstruct amino acid from: {0}")]
    UnknownAminoAcid(char),
}
