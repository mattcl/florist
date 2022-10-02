use std::{fmt::Display, str::FromStr};

use crate::Error;

pub trait Codon: TryFrom<(char, char, char)> {
    fn from_tuple_unchecked(v: (char, char, char)) -> Self;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DNACodon(pub char, pub char, pub char);

impl Codon for DNACodon {
    /// Only intended for supporting construction from rna sequences
    fn from_tuple_unchecked(value: (char, char, char)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl TryFrom<(char, char, char)> for DNACodon {
    type Error = Error;

    fn try_from(value: (char, char, char)) -> Result<Self, Self::Error> {
        if !matches!(value.0, 'A' | 'T' | 'C' | 'G') {
            return Err(Error::InvalidDNACodon(value.0));
        }

        if !matches!(value.1, 'A' | 'T' | 'C' | 'G') {
            return Err(Error::InvalidDNACodon(value.1));
        }

        if !matches!(value.2, 'A' | 'T' | 'C' | 'G') {
            return Err(Error::InvalidDNACodon(value.2));
        }

        Ok(Self(value.0, value.1, value.2))
    }
}

impl FromStr for DNACodon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(Error::CodonWrongLength(s.into()));
        }

        let chars: Vec<_> = s.chars().collect();

        DNACodon::try_from((chars[0], chars[1], chars[2]))
    }
}

impl Display for DNACodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct RNACodon(pub char, pub char, pub char);

impl Codon for RNACodon {
    /// Only intended for supporting construction from rna sequences
    fn from_tuple_unchecked(value: (char, char, char)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl TryFrom<(char, char, char)> for RNACodon {
    type Error = Error;

    fn try_from(value: (char, char, char)) -> Result<Self, Self::Error> {
        if !matches!(value.0, 'A' | 'U' | 'C' | 'G') {
            return Err(Error::InvalidRNACodon(value.0));
        }

        if !matches!(value.1, 'A' | 'U' | 'C' | 'G') {
            return Err(Error::InvalidRNACodon(value.1));
        }

        if !matches!(value.2, 'A' | 'U' | 'C' | 'G') {
            return Err(Error::InvalidRNACodon(value.2));
        }

        Ok(Self(value.0, value.1, value.2))
    }
}

impl FromStr for RNACodon {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(Error::CodonWrongLength(s.into()));
        }

        let chars: Vec<_> = s.chars().collect();

        RNACodon::try_from((chars[0], chars[1], chars[2]))
    }
}

impl Display for RNACodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}
