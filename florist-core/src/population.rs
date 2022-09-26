use crate::Error;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct SingleGenePopulation {
    /// AA-AA
    dd: u32,

    /// AA-Aa
    dh: u32,

    /// AA-aa
    dr: u32,

    /// Aa-Aa
    hh: u32,

    /// Aa-aa
    hr: u32,

    /// aa-aa
    rr: u32,
}

impl SingleGenePopulation {
    pub fn expected_dominant_offspring(&self, litter_size: u32) -> f64 {
        ((self.dd + self.dh + self.dr) as f64 + (self.hh as f64 * 0.75) + (self.hr as f64 * 0.5))
            * litter_size as f64
    }
}

impl TryFrom<Vec<u32>> for SingleGenePopulation {
    type Error = Error;

    fn try_from(value: Vec<u32>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Vec<u32>> for SingleGenePopulation {
    type Error = Error;

    fn try_from(value: &Vec<u32>) -> Result<Self, Self::Error> {
        if value.len() != 6 {
            return Err(Error::WrongNumberOfValues {
                desired: 6,
                actual: value.len(),
            });
        }

        Ok(Self {
            dd: value[0],
            dh: value[1],
            dr: value[2],
            hh: value[3],
            hr: value[4],
            rr: value[5],
        })
    }
}
