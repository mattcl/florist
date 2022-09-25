use crate::{DNACodon, Error, RNACodon};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    AsparticAcid,
    Cysteine,
    GlutamicAcid,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
    // special stop values
    Ochre,
    Amber,
    Opal,
}

impl AminoAcid {
    pub fn abbreviation(&self) -> char {
        match self {
            Self::Alanine => 'A',
            Self::Arginine => 'R',
            Self::Asparagine => 'N',
            Self::AsparticAcid => 'D',
            Self::Cysteine => 'C',
            Self::GlutamicAcid => 'E',
            Self::Glutamine => 'Q',
            Self::Glycine => 'G',
            Self::Histidine => 'H',
            Self::Isoleucine => 'I',
            Self::Leucine => 'L',
            Self::Lysine => 'K',
            Self::Methionine => 'M',
            Self::Phenylalanine => 'F',
            Self::Proline => 'P',
            Self::Serine => 'S',
            Self::Threonine => 'T',
            Self::Tryptophan => 'W',
            Self::Tyrosine => 'Y',
            Self::Valine => 'V',
            _ => 'X',
        }
    }
}

impl TryFrom<DNACodon> for AminoAcid {
    type Error = Error;

    fn try_from(value: DNACodon) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&DNACodon> for AminoAcid {
    type Error = Error;

    fn try_from(value: &DNACodon) -> Result<Self, Self::Error> {
        // yeah, this is a little nuts. My reasoning was that there would be
        // fewer overall matches to perform if we went with one char at a time.
        Ok(match value.0 {
            'T' => match value.1 {
                'T' => match value.2 {
                    'T' | 'C' => Self::Phenylalanine,
                    'A' | 'G' => Self::Leucine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'C' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Serine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'A' => match value.2 {
                    'T' | 'C' => Self::Tyrosine,
                    'A' => Self::Ochre,
                    'G' => Self::Amber,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'G' => match value.2 {
                    'T' | 'C' => Self::Cysteine,
                    'A' => Self::Opal,
                    'G' => Self::Tryptophan,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                _ => return Err(Error::UnknownDnaCodon(*value)),
            },
            'C' => match value.1 {
                'T' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Leucine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'C' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Proline,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'A' => match value.2 {
                    'T' | 'C' => Self::Histidine,
                    'A' | 'G' => Self::Glutamine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'G' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Arginine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                _ => return Err(Error::UnknownDnaCodon(*value)),
            },
            'A' => match value.1 {
                'T' => match value.2 {
                    'T' | 'C' | 'A' => Self::Isoleucine,
                    'G' => Self::Methionine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'C' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Threonine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'A' => match value.2 {
                    'T' | 'C' => Self::Asparagine,
                    'A' | 'G' => Self::Lysine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'G' => match value.2 {
                    'T' | 'C' => Self::Serine,
                    'A' | 'G' => Self::Arginine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                _ => return Err(Error::UnknownDnaCodon(*value)),
            },
            'G' => match value.1 {
                'T' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Valine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'C' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Alanine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'A' => match value.2 {
                    'T' | 'C' => Self::AsparticAcid,
                    'A' | 'G' => Self::GlutamicAcid,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                'G' => match value.2 {
                    'T' | 'C' | 'A' | 'G' => Self::Glycine,
                    _ => return Err(Error::UnknownDnaCodon(*value)),
                },
                _ => return Err(Error::UnknownDnaCodon(*value)),
            },
            _ => return Err(Error::UnknownDnaCodon(*value)),
        })
    }
}

impl TryFrom<RNACodon> for AminoAcid {
    type Error = Error;

    fn try_from(value: RNACodon) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&RNACodon> for AminoAcid {
    type Error = Error;

    fn try_from(value: &RNACodon) -> Result<Self, Self::Error> {
        // yeah, this is a little nuts. My reasoning was that there would be
        // fewer overall matches to perform if we went with one char at a time.
        Ok(match value.0 {
            'U' => match value.1 {
                'U' => match value.2 {
                    'U' | 'C' => Self::Phenylalanine,
                    'A' | 'G' => Self::Leucine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'C' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Serine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'A' => match value.2 {
                    'U' | 'C' => Self::Tyrosine,
                    'A' => Self::Ochre,
                    'G' => Self::Amber,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'G' => match value.2 {
                    'U' | 'C' => Self::Cysteine,
                    'A' => Self::Opal,
                    'G' => Self::Tryptophan,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                _ => return Err(Error::UnknownRnaCodon(*value)),
            },
            'C' => match value.1 {
                'U' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Leucine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'C' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Proline,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'A' => match value.2 {
                    'U' | 'C' => Self::Histidine,
                    'A' | 'G' => Self::Glutamine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'G' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Arginine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                _ => return Err(Error::UnknownRnaCodon(*value)),
            },
            'A' => match value.1 {
                'U' => match value.2 {
                    'U' | 'C' | 'A' => Self::Isoleucine,
                    'G' => Self::Methionine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'C' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Threonine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'A' => match value.2 {
                    'U' | 'C' => Self::Asparagine,
                    'A' | 'G' => Self::Lysine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'G' => match value.2 {
                    'U' | 'C' => Self::Serine,
                    'A' | 'G' => Self::Arginine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                _ => return Err(Error::UnknownRnaCodon(*value)),
            },
            'G' => match value.1 {
                'U' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Valine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'C' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Alanine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'A' => match value.2 {
                    'U' | 'C' => Self::AsparticAcid,
                    'A' | 'G' => Self::GlutamicAcid,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                'G' => match value.2 {
                    'U' | 'C' | 'A' | 'G' => Self::Glycine,
                    _ => return Err(Error::UnknownRnaCodon(*value)),
                },
                _ => return Err(Error::UnknownRnaCodon(*value)),
            },
            _ => return Err(Error::UnknownRnaCodon(*value)),
        })
    }
}

#[cfg(test)]
mod tests {
    mod abbreviations {
        use super::super::*;

        macro_rules! abbrev_test {
            ($name:ident, $variant:path, $expected:literal) => {
                #[test]
                fn $name() {
                    let res = $variant.abbreviation();
                    assert_eq!(res, $expected);
                }
            };
        }

        abbrev_test!(alanine, AminoAcid::Alanine, 'A');
        abbrev_test!(arginine, AminoAcid::Arginine, 'R');
        abbrev_test!(asparagine, AminoAcid::Asparagine, 'N');
        abbrev_test!(aspartic_acid, AminoAcid::AsparticAcid, 'D');
        abbrev_test!(cysteine, AminoAcid::Cysteine, 'C');
        abbrev_test!(glutamic_acid, AminoAcid::GlutamicAcid, 'E');
        abbrev_test!(glutamine, AminoAcid::Glutamine, 'Q');
        abbrev_test!(glycine, AminoAcid::Glycine, 'G');
        abbrev_test!(histidine, AminoAcid::Histidine, 'H');
        abbrev_test!(isoleucine, AminoAcid::Isoleucine, 'I');
        abbrev_test!(leucine, AminoAcid::Leucine, 'L');
        abbrev_test!(lysine, AminoAcid::Lysine, 'K');
        abbrev_test!(methionine, AminoAcid::Methionine, 'M');
        abbrev_test!(phenyalanine, AminoAcid::Phenylalanine, 'F');
        abbrev_test!(proline, AminoAcid::Proline, 'P');
        abbrev_test!(serine, AminoAcid::Serine, 'S');
        abbrev_test!(threonine, AminoAcid::Threonine, 'T');
        abbrev_test!(tryptophan, AminoAcid::Tryptophan, 'W');
        abbrev_test!(tyrosine, AminoAcid::Tyrosine, 'Y');
        abbrev_test!(valine, AminoAcid::Valine, 'V');
        abbrev_test!(ochre, AminoAcid::Ochre, 'X');
        abbrev_test!(amber, AminoAcid::Amber, 'X');
        abbrev_test!(opal, AminoAcid::Opal, 'X');
    }

    mod from_rna_codon {
        use super::super::*;
        use std::str::FromStr;

        macro_rules! codon_test {
            ($rna:ident, $expected:expr) => {
                #[test]
                #[allow(non_snake_case)]
                fn $rna() {
                    let c = RNACodon::from_str(stringify!($rna)).unwrap();
                    let res = AminoAcid::try_from(&c).unwrap();
                    assert_eq!(res, $expected);
                }
            };
        }

        codon_test!(AAA, AminoAcid::Lysine);
        codon_test!(AAG, AminoAcid::Lysine);
        codon_test!(AAC, AminoAcid::Asparagine);
        codon_test!(AAU, AminoAcid::Asparagine);
        codon_test!(ACA, AminoAcid::Threonine);
        codon_test!(ACC, AminoAcid::Threonine);
        codon_test!(ACG, AminoAcid::Threonine);
        codon_test!(ACU, AminoAcid::Threonine);
        codon_test!(AGA, AminoAcid::Arginine);
        codon_test!(AGG, AminoAcid::Arginine);
        codon_test!(CGA, AminoAcid::Arginine);
        codon_test!(CGC, AminoAcid::Arginine);
        codon_test!(CGG, AminoAcid::Arginine);
        codon_test!(CGU, AminoAcid::Arginine);
        codon_test!(AGC, AminoAcid::Serine);
        codon_test!(AGU, AminoAcid::Serine);
        codon_test!(UCA, AminoAcid::Serine);
        codon_test!(UCC, AminoAcid::Serine);
        codon_test!(UCG, AminoAcid::Serine);
        codon_test!(UCU, AminoAcid::Serine);
        codon_test!(AUA, AminoAcid::Isoleucine);
        codon_test!(AUC, AminoAcid::Isoleucine);
        codon_test!(AUU, AminoAcid::Isoleucine);
        codon_test!(AUG, AminoAcid::Methionine);
        codon_test!(CAA, AminoAcid::Glutamine);
        codon_test!(CAG, AminoAcid::Glutamine);
        codon_test!(CAC, AminoAcid::Histidine);
        codon_test!(CAU, AminoAcid::Histidine);
        codon_test!(CCA, AminoAcid::Proline);
        codon_test!(CCC, AminoAcid::Proline);
        codon_test!(CCG, AminoAcid::Proline);
        codon_test!(CCU, AminoAcid::Proline);
        codon_test!(CUA, AminoAcid::Leucine);
        codon_test!(CUC, AminoAcid::Leucine);
        codon_test!(CUG, AminoAcid::Leucine);
        codon_test!(CUU, AminoAcid::Leucine);
        codon_test!(UUA, AminoAcid::Leucine);
        codon_test!(UUG, AminoAcid::Leucine);
        codon_test!(GAA, AminoAcid::GlutamicAcid);
        codon_test!(GAG, AminoAcid::GlutamicAcid);
        codon_test!(GAC, AminoAcid::AsparticAcid);
        codon_test!(GAU, AminoAcid::AsparticAcid);
        codon_test!(GCA, AminoAcid::Alanine);
        codon_test!(GCC, AminoAcid::Alanine);
        codon_test!(GCG, AminoAcid::Alanine);
        codon_test!(GCU, AminoAcid::Alanine);
        codon_test!(GGA, AminoAcid::Glycine);
        codon_test!(GGC, AminoAcid::Glycine);
        codon_test!(GGG, AminoAcid::Glycine);
        codon_test!(GGU, AminoAcid::Glycine);
        codon_test!(GUA, AminoAcid::Valine);
        codon_test!(GUC, AminoAcid::Valine);
        codon_test!(GUG, AminoAcid::Valine);
        codon_test!(GUU, AminoAcid::Valine);
        codon_test!(UAC, AminoAcid::Tyrosine);
        codon_test!(UAU, AminoAcid::Tyrosine);
        codon_test!(UGC, AminoAcid::Cysteine);
        codon_test!(UGU, AminoAcid::Cysteine);
        codon_test!(UGG, AminoAcid::Tryptophan);
        codon_test!(UUC, AminoAcid::Phenylalanine);
        codon_test!(UUU, AminoAcid::Phenylalanine);
        codon_test!(UAA, AminoAcid::Ochre);
        codon_test!(UAG, AminoAcid::Amber);
        codon_test!(UGA, AminoAcid::Opal);
    }

    mod from_dna_codon {
        use super::super::*;
        use std::str::FromStr;

        macro_rules! codon_test {
            ($rna:ident, $expected:expr) => {
                #[test]
                #[allow(non_snake_case)]
                fn $rna() {
                    let c = DNACodon::from_str(stringify!($rna)).unwrap();
                    let res = AminoAcid::try_from(&c).unwrap();
                    assert_eq!(res, $expected);
                }
            };
        }

        codon_test!(AAA, AminoAcid::Lysine);
        codon_test!(AAG, AminoAcid::Lysine);
        codon_test!(AAC, AminoAcid::Asparagine);
        codon_test!(AAT, AminoAcid::Asparagine);
        codon_test!(ACA, AminoAcid::Threonine);
        codon_test!(ACC, AminoAcid::Threonine);
        codon_test!(ACG, AminoAcid::Threonine);
        codon_test!(ACT, AminoAcid::Threonine);
        codon_test!(AGA, AminoAcid::Arginine);
        codon_test!(AGG, AminoAcid::Arginine);
        codon_test!(CGA, AminoAcid::Arginine);
        codon_test!(CGC, AminoAcid::Arginine);
        codon_test!(CGG, AminoAcid::Arginine);
        codon_test!(CGT, AminoAcid::Arginine);
        codon_test!(AGC, AminoAcid::Serine);
        codon_test!(AGT, AminoAcid::Serine);
        codon_test!(TCA, AminoAcid::Serine);
        codon_test!(TCC, AminoAcid::Serine);
        codon_test!(TCG, AminoAcid::Serine);
        codon_test!(TCT, AminoAcid::Serine);
        codon_test!(ATA, AminoAcid::Isoleucine);
        codon_test!(ATC, AminoAcid::Isoleucine);
        codon_test!(ATT, AminoAcid::Isoleucine);
        codon_test!(ATG, AminoAcid::Methionine);
        codon_test!(CAA, AminoAcid::Glutamine);
        codon_test!(CAG, AminoAcid::Glutamine);
        codon_test!(CAC, AminoAcid::Histidine);
        codon_test!(CAT, AminoAcid::Histidine);
        codon_test!(CCA, AminoAcid::Proline);
        codon_test!(CCC, AminoAcid::Proline);
        codon_test!(CCG, AminoAcid::Proline);
        codon_test!(CCT, AminoAcid::Proline);
        codon_test!(CTA, AminoAcid::Leucine);
        codon_test!(CTC, AminoAcid::Leucine);
        codon_test!(CTG, AminoAcid::Leucine);
        codon_test!(CTT, AminoAcid::Leucine);
        codon_test!(TTA, AminoAcid::Leucine);
        codon_test!(TTG, AminoAcid::Leucine);
        codon_test!(GAA, AminoAcid::GlutamicAcid);
        codon_test!(GAG, AminoAcid::GlutamicAcid);
        codon_test!(GAC, AminoAcid::AsparticAcid);
        codon_test!(GAT, AminoAcid::AsparticAcid);
        codon_test!(GCA, AminoAcid::Alanine);
        codon_test!(GCC, AminoAcid::Alanine);
        codon_test!(GCG, AminoAcid::Alanine);
        codon_test!(GCT, AminoAcid::Alanine);
        codon_test!(GGA, AminoAcid::Glycine);
        codon_test!(GGC, AminoAcid::Glycine);
        codon_test!(GGG, AminoAcid::Glycine);
        codon_test!(GGT, AminoAcid::Glycine);
        codon_test!(GTA, AminoAcid::Valine);
        codon_test!(GTC, AminoAcid::Valine);
        codon_test!(GTG, AminoAcid::Valine);
        codon_test!(GTT, AminoAcid::Valine);
        codon_test!(TAC, AminoAcid::Tyrosine);
        codon_test!(TAT, AminoAcid::Tyrosine);
        codon_test!(TGC, AminoAcid::Cysteine);
        codon_test!(TGT, AminoAcid::Cysteine);
        codon_test!(TGG, AminoAcid::Tryptophan);
        codon_test!(TTC, AminoAcid::Phenylalanine);
        codon_test!(TTT, AminoAcid::Phenylalanine);
        codon_test!(TAA, AminoAcid::Ochre);
        codon_test!(TAG, AminoAcid::Amber);
        codon_test!(TGA, AminoAcid::Opal);
    }
}
