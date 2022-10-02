//! Defines special string sequences and tools for working with them.
use std::{
    fmt::Display,
    hash::Hash,
    iter::Map,
    marker::PhantomData,
    ops::Deref,
    str::{Chars, FromStr},
};

use itertools::{join, Itertools, Tuples};
use rustc_hash::FxHashMap;

use crate::{codon::Codon, AminoAcid, DNACodon, Error, RNACodon};

pub trait Sequence:
    FromStr
    + TryFrom<String>
    + Display
    + std::fmt::Debug
    + Clone
    + Eq
    + PartialEq
    + Hash
    + Deref<Target = String>
{
    const SYMBOLS: &'static str;

    fn new_unchecked(val: String) -> Self;

    fn is_valid_char(ch: char) -> bool;

    fn symbols() -> Chars<'static> {
        Self::SYMBOLS.chars()
    }

    fn frames(&self) -> Vec<Frame<Self>> {
        (0..3)
            .filter_map(|offset| Frame::new(self, offset))
            .collect()
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct DNASequence(String);

impl DNASequence {
    pub fn complement(&self) -> Self {
        Self(
            self.chars()
                .map(|ch| match ch {
                    'A' => 'T',
                    'T' => 'A',
                    'C' => 'G',
                    'G' => 'C',
                    _ => unreachable!(),
                })
                .collect::<String>(),
        )
    }

    pub fn reverse_complement(&self) -> Self {
        Self(
            self.chars()
                .rev()
                .map(|ch| match ch {
                    'A' => 'T',
                    'T' => 'A',
                    'C' => 'G',
                    'G' => 'C',
                    _ => unreachable!(),
                })
                .collect::<String>(),
        )
    }

    pub fn subsequence_unchecked(&self, start: usize, end: usize) -> Self {
        Self(String::from_utf8_lossy(&self.as_bytes()[start..end]).into())
    }

    pub fn open_frames(&self) -> Vec<Frame<Self>> {
        let needle = "ATG";
        self.0
            .match_indices(needle)
            .filter_map(|(i, _)| Frame::new(self, i))
            .collect()
    }
}

impl Sequence for DNASequence {
    const SYMBOLS: &'static str = "ACGT";

    fn new_unchecked(val: String) -> Self {
        Self(val)
    }

    fn is_valid_char(ch: char) -> bool {
        matches!(ch, 'A' | 'C' | 'G' | 'T')
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
        if s.is_empty() {
            return Err(Error::EmptySequence);
        }

        for ch in s.chars() {
            if !Self::is_valid_char(ch) {
                return Err(Error::InvalidSequenceCharacter(ch));
            }
        }

        Ok(Self(s.into()))
    }
}

impl TryFrom<String> for DNASequence {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::EmptySequence);
        }

        for ch in value.chars() {
            if !Self::is_valid_char(ch) {
                return Err(Error::InvalidSequenceCharacter(ch));
            }
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct RNASequence(String);

impl RNASequence {
    pub fn open_frames(&self) -> Vec<Frame<Self>> {
        let needle = "AUG";
        self.0
            .match_indices(needle)
            .filter_map(|(i, _)| Frame::new(self, i))
            .collect()
    }
}

impl Sequence for RNASequence {
    const SYMBOLS: &'static str = "ACGU";

    fn new_unchecked(val: String) -> Self {
        Self(val)
    }

    fn is_valid_char(ch: char) -> bool {
        matches!(ch, 'A' | 'C' | 'G' | 'U')
    }
}

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
        if s.is_empty() {
            return Err(Error::EmptySequence);
        }

        for ch in s.chars() {
            if !Self::is_valid_char(ch) {
                return Err(Error::InvalidSequenceCharacter(ch));
            }
        }

        Ok(Self(s.into()))
    }
}

impl TryFrom<String> for RNASequence {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::EmptySequence);
        }

        for ch in value.chars() {
            if !Self::is_valid_char(ch) {
                return Err(Error::InvalidSequenceCharacter(ch));
            }
        }

        Ok(Self(value))
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

pub trait GCContent {
    fn gc_content(&self) -> f64;
}

impl GCContent for DNASequence {
    fn gc_content(&self) -> f64 {
        let numer = self.chars().filter(|ch| *ch == 'G' || *ch == 'C').count() as f64;
        let denom = self.len() as f64;

        numer / denom
    }
}

impl GCContent for RNASequence {
    fn gc_content(&self) -> f64 {
        let numer = self.chars().filter(|ch| *ch == 'G' || *ch == 'C').count() as f64;
        let denom = self.len() as f64;

        numer / denom
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct ProteinSequence(String);

impl ProteinSequence {
    pub fn monoisotopic_mass(&self) -> f64 {
        self.chars()
            // since we validate the protein construction, we should not be in
            // a situation where we fail here
            .map(|ch| AminoAcid::try_from(ch).expect("Should not have been possible"))
            .map(|acid| acid.monoisotopic_mass())
            .sum()
    }
}

impl Sequence for ProteinSequence {
    const SYMBOLS: &'static str = "ACDEFGHIKLMNPQRSTVWY";

    fn new_unchecked(val: String) -> Self {
        Self(val)
    }

    fn is_valid_char(ch: char) -> bool {
        ch.is_ascii_uppercase()
            && ch.is_ascii_alphabetic()
            && !matches!(ch, 'B' | 'J' | 'O' | 'U' | 'X' | 'Z')
    }
}

impl Deref for ProteinSequence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ProteinSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ProteinSequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::EmptySequence);
        }

        for ch in s.chars() {
            if !Self::is_valid_char(ch) {
                return Err(Error::InvalidSequenceCharacter(ch));
            }
        }

        Ok(Self(s.into()))
    }
}

impl TryFrom<String> for ProteinSequence {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::EmptySequence);
        }

        for ch in value.chars() {
            if !Self::is_valid_char(ch) {
                return Err(Error::InvalidSequenceCharacter(ch));
            }
        }

        Ok(Self(value))
    }
}

pub trait HammingDistance<Other = Self> {
    type Error;

    fn hamming_distance(&self, other: &Other) -> Result<u64, Self::Error>;
}

impl<T: Sequence> HammingDistance for T {
    type Error = Error;

    fn hamming_distance(&self, other: &Self) -> Result<u64, Self::Error> {
        if self.len() != other.len() {
            return Err(Error::NotEqualLength);
        }

        let mut count = 0;
        let mut pairs = self.chars().zip(other.chars());

        while let Some((my, their)) = pairs.next() {
            if my != their {
                count += 1;
            }
        }

        Ok(count)
    }
}

pub trait GeneticSequence {
    type Codon: Codon + TryInto<AminoAcid, Error = Error>;

    fn codons(
        &self,
    ) -> Map<Tuples<Chars<'_>, (char, char, char)>, fn((char, char, char)) -> Self::Codon>;

    fn to_protein(&self) -> Result<ProteinSequence, <Self::Codon as TryInto<AminoAcid>>::Error> {
        let mut codons = self.codons();

        let mut output = String::new();
        while let Some(cur) = codons.next() {
            let marker: AminoAcid = cur.try_into()?;
            if marker.is_start() {
                output.push(marker.abbreviation());

                while let Some(next) = codons.next() {
                    let acid: AminoAcid = next.try_into()?;
                    if acid.is_stop() {
                        return Ok(ProteinSequence(output));
                    } else {
                        output.push(acid.abbreviation());
                    }
                }
            }
        }

        Err(Error::NoValidProtein)
    }
}

impl GeneticSequence for DNASequence {
    type Codon = DNACodon;

    fn codons(
        &self,
    ) -> Map<Tuples<Chars<'_>, (char, char, char)>, fn((char, char, char)) -> Self::Codon> {
        self.chars().tuples().map(DNACodon::from_tuple_unchecked)
    }
}

impl GeneticSequence for RNASequence {
    type Codon = RNACodon;

    fn codons(
        &self,
    ) -> Map<Tuples<Chars<'_>, (char, char, char)>, fn((char, char, char)) -> Self::Codon> {
        self.chars().tuples().map(RNACodon::from_tuple_unchecked)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Frame<'a, T> {
    sequence: &'a str,
    _marker: PhantomData<T>,
}

impl<'a, T: Sequence> Frame<'a, T> {
    fn new(sequence: &'a T, offset: usize) -> Option<Self> {
        let slice = sequence.get(offset..)?;

        if slice.is_empty() {
            return None;
        }

        Some(Self {
            sequence: slice,
            _marker: PhantomData,
        })
    }
}

impl<'a, T: Sequence + GeneticSequence> GeneticSequence for Frame<'a, T> {
    type Codon = <T as GeneticSequence>::Codon;

    fn codons(
        &self,
    ) -> Map<Tuples<Chars<'_>, (char, char, char)>, fn((char, char, char)) -> Self::Codon> {
        self.sequence
            .chars()
            .tuples()
            .map(<T as GeneticSequence>::Codon::from_tuple_unchecked)
    }
}

impl TryFrom<DNASequence> for ProteinSequence {
    type Error = Error;

    fn try_from(value: DNASequence) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&DNASequence> for ProteinSequence {
    type Error = Error;

    fn try_from(value: &DNASequence) -> Result<Self, Self::Error> {
        value.to_protein()
    }
}

impl TryFrom<RNASequence> for ProteinSequence {
    type Error = Error;

    fn try_from(value: RNASequence) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&RNASequence> for ProteinSequence {
    type Error = Error;

    fn try_from(value: &RNASequence) -> Result<Self, Self::Error> {
        value.to_protein()
    }
}

impl<'a, T: Sequence + GeneticSequence> TryFrom<Frame<'a, T>> for ProteinSequence {
    type Error = Error;

    fn try_from(value: Frame<'a, T>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<'a, T: Sequence + GeneticSequence> TryFrom<&Frame<'a, T>> for ProteinSequence {
    type Error = Error;

    fn try_from(value: &Frame<'a, T>) -> Result<Self, Self::Error> {
        value.to_protein()
    }
}

pub trait Motif {
    fn motif_lcoations(&self, motif: &Self) -> Vec<usize>;
}

impl<T: Sequence> Motif for T {
    fn motif_lcoations(&self, motif: &Self) -> Vec<usize> {
        // we know we only allow a small subset of ascii chars, so the bytes in
        // the sequence and motif are going to be individual chars.
        let needle = motif.as_bytes();
        let haystack = self.as_bytes();

        let len = haystack.len();
        let size = needle.len();
        let mut indicies = Vec::new();

        if len < size {
            return indicies;
        }

        for i in 0..(len - size + 1) {
            if &haystack[i..(i + size)] == needle {
                indicies.push(i);
            }
        }

        indicies
    }
}

pub struct Consensus<T: Sequence> {
    sequence: T,
    frequencies: FxHashMap<char, Vec<usize>>,
}

impl<'a, T: Sequence + 'a> Consensus<T> {
    pub fn sequence(&self) -> &T {
        &self.sequence
    }

    pub fn frequencies(&self) -> &FxHashMap<char, Vec<usize>> {
        &self.frequencies
    }

    pub fn try_from_iter<I: Iterator<Item = &'a T>>(iter: I) -> Result<Self, Error> {
        let seqs: Vec<_> = iter.map(|s| s.as_bytes()).collect();

        if seqs.is_empty() {
            return Err(Error::NoSequences);
        }

        if !seqs.iter().map(|s| s.len()).all_equal() {
            // inequal length error
            return Err(Error::NotEqualLength);
        }

        let len = seqs[0].len();

        let mut frequencies: FxHashMap<char, Vec<usize>> =
            FxHashMap::from_iter(T::symbols().map(|s| (s, vec![0; len])));

        let mut sequence = String::with_capacity(len);

        for i in 0..seqs[0].len() {
            let mut counts: FxHashMap<char, usize> = FxHashMap::default();

            for s in 0..seqs.len() {
                // we're okay casting because we _know_ our sequences are ascii
                counts
                    .entry(seqs[s][i] as char)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }

            // we know this can't be empty, so unwrapping is fine
            let (ch, _) = counts.iter().sorted().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            sequence.push(*ch);

            for (ch, count) in counts.iter() {
                frequencies.entry(*ch).and_modify(|e| e[i] = *count);
            }
        }

        Ok(Self {
            sequence: T::new_unchecked(sequence),
            frequencies,
        })
    }
}

impl<T: Sequence> Display for Consensus<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let freqs = self
            .frequencies
            .iter()
            .sorted_by_key(|e| e.0)
            .map(|(k, v)| format!("{}: {}", k, join(v, " ")))
            .join("\n");
        write!(f, "{}\n{}", &self.sequence, freqs)
    }
}
