use std::{fmt::Display, ops::Deref, str::FromStr};

use crate::Error;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct DNASequence(String);

impl DNASequence {
    pub fn complement(&self) -> Self {
        Self(
            self
                .chars()
                .map(|ch| {
                    match ch {
                        'A' => 'T',
                        'T' => 'A',
                        'C' => 'G',
                        'G' => 'C',
                        _ => unreachable!()
                    }
                })
                .collect::<String>()
        )
    }

    pub fn reverse_complement(&self) -> Self {
        Self(
            self
                .chars()
                .rev()
                .map(|ch| {
                    match ch {
                        'A' => 'T',
                        'T' => 'A',
                        'C' => 'G',
                        'G' => 'C',
                        _ => unreachable!()
                    }
                })
                .collect::<String>()
        )
    }
}

impl Deref for DNASequence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for DNASequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DNASequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for ch in s.chars() {
            if !(ch == 'A' || ch == 'C' || ch == 'G' || ch == 'T') {
                return Err(Error::InvalidDNASequence(ch));
            }
        }

        Ok(Self(s.into()))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct RNASequence(String);

impl Deref for RNASequence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for RNASequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for RNASequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for ch in s.chars() {
            if !(ch == 'A' || ch == 'C' || ch == 'G' || ch == 'U') {
                return Err(Error::InvalidRNASequence(ch));
            }
        }

        Ok(Self(s.into()))
    }
}

// Conversions

impl From<DNASequence> for RNASequence {
    fn from(val: DNASequence) -> Self {
        RNASequence(val.replace('T', "U"))
    }
}

impl From<RNASequence> for DNASequence {
    fn from(val: RNASequence) -> Self {
        DNASequence(val.replace('U', "T"))
    }
}
