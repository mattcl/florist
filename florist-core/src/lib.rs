pub mod amino;
pub mod codon;
pub mod population;
pub mod sequence;

pub use amino::AminoAcid;
pub use codon::{DNACodon, RNACodon};
pub use population::SingleGenePopulation;
pub use sequence::{
    Consensus, DNASequence, GCContent, GeneticSequence, HammingDistance, Motif, ProteinSequence,
    RNASequence, Sequence,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid character in sequence: {0}")]
    InvalidSequenceCharacter(char),

    #[error("Empty strings are not valid sequences")]
    EmptySequence,

    #[error("Sequences must be equal in length")]
    NotEqualLength,

    #[error("Empty list of sequences")]
    NoSequences,

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

    #[error("Wrong number of values. Expected {desired} but got {actual}")]
    WrongNumberOfValues { desired: usize, actual: usize },
}
