use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("No sequences defined in file")]
    NoSequences,

    #[error("Empty sequence detected in file")]
    EmptySequence,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct MultiFasta(HashMap<String, String>);

impl Deref for MultiFasta {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MultiFasta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for MultiFasta {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();

        let mut sequences = Self::default();

        while let Some(line) = lines.next() {
            if line.starts_with('>') {
                // we have a new sequence, and the current line is the
                // description
                let description = line.split_at(1).1.trim().to_owned();
                let mut sequence_parts = Vec::new();

                while let Some(seq) = lines.peek() {
                    if seq.starts_with('>') {
                        break;
                    }

                    // we can unwerap because we checked for presence as the
                    // loop condition
                    sequence_parts.push(lines.next().unwrap().trim());
                }

                if sequence_parts.is_empty() {
                    return Err(Error::EmptySequence);
                }

                let sequence = sequence_parts.join("");

                sequences.insert(description, sequence);
            }
        }

        if sequences.is_empty() {
            return Err(Error::NoSequences);
        }

        Ok(sequences)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct OrderedMultiFasta(Vec<(String, String)>);

impl Deref for OrderedMultiFasta {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OrderedMultiFasta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for OrderedMultiFasta {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();

        let mut sequences = Self::default();

        while let Some(line) = lines.next() {
            if line.starts_with('>') {
                // we have a new sequence, and the current line is the
                // description
                let description = line.split_at(1).1.trim().to_owned();
                let mut sequence_parts = Vec::new();

                while let Some(seq) = lines.peek() {
                    if seq.starts_with('>') {
                        break;
                    }

                    // we can unwerap because we checked for presence as the
                    // loop condition
                    sequence_parts.push(lines.next().unwrap().trim());
                }

                if sequence_parts.is_empty() {
                    return Err(Error::EmptySequence);
                }

                let sequence = sequence_parts.join("");

                sequences.push((description, sequence));
            }
        }

        if sequences.is_empty() {
            return Err(Error::NoSequences);
        }

        Ok(sequences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let input = "
>Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG

>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";

        let parsed = MultiFasta::from_str(input).expect("Failed to parse valid file");
        assert_eq!(parsed.len(), 3);

        let mid = parsed.get("Rosalind_5959").unwrap();
        let expected =
            "CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC";
        assert_eq!(mid, expected);
    }

    #[test]
    fn errors_if_sequence_empty() {
        let input = "
>Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808";

        let parsed = MultiFasta::from_str(input);
        assert_eq!(parsed, Err(Error::EmptySequence));
    }

    #[test]
    fn errors_if_no_description() {
        let input = "
Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC";

        let parsed = MultiFasta::from_str(input);
        assert_eq!(parsed, Err(Error::NoSequences));
    }
}
