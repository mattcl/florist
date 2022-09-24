// Pk Pm Pn
// Pk * P(k-1)

use std::str::FromStr;

use anyhow::bail;
use florist_plumbing::Problem;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Population {
    homozygous_dominant: u32,
    homozygous_recessive: u32,
    heterozygous: u32,
}

impl Population {
    pub fn p_offspring_dominant(&self) -> f64 {
        let size = self.size();

        // I suppose we could consider this an error.
        if size < 2 {
            return 0.0;
        }

        let p_dd = (self.homozygous_dominant as f64 / size as f64)
            * ((self.homozygous_dominant - 1) as f64 / (size - 1) as f64);
        let p_hh = (self.heterozygous as f64 / size as f64)
            * ((self.heterozygous - 1) as f64 / (size - 1) as f64);

        let p_dr = 2.0
            * (self.homozygous_dominant as f64 / size as f64)
            * (self.homozygous_recessive as f64 / (size - 1) as f64);
        let p_dh = 2.0
            * (self.homozygous_dominant as f64 / size as f64)
            * (self.heterozygous as f64 / (size - 1) as f64);
        let p_rh = 2.0
            * (self.homozygous_recessive as f64 / size as f64)
            * (self.heterozygous as f64 / (size - 1) as f64);

        p_dd + p_dr + p_dh + p_hh * 0.75 + p_rh * 0.5
    }

    pub fn size(&self) -> u32 {
        self.homozygous_dominant + self.homozygous_recessive + self.heterozygous
    }
}

impl FromStr for Population {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        if values.len() != 3 {
            bail!("Invalid number of values in input: {}", s);
        }

        Ok(Population {
            homozygous_dominant: values[0],
            homozygous_recessive: values[2],
            heterozygous: values[1],
        })
    }
}

pub struct MendelsFirstLaw;

impl Problem for MendelsFirstLaw {
    type Error = anyhow::Error;
    type Input = Population;
    type Output = f64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.p_offspring_dominant())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offspring_dominant() {
        let raw = "2 2 2";
        let input = Population::from_str(raw).unwrap();
        let output = MendelsFirstLaw::solve(input).unwrap();
        assert!(output - 0.7833333333333333 < f64::EPSILON);
    }
}
