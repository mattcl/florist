use florist_core::sequence::ProteinSequence;
use florist_plumbing::Problem;

pub struct CalculatingProteinMass;

impl Problem for CalculatingProteinMass {
    type Error = florist_core::Error;
    type Input = ProteinSequence;
    type Output = f64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.monoisotopic_mass())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn protein_mass() {
        let raw = "SKADYEK";
        let input = ProteinSequence::from_str(raw).unwrap();
        let output = CalculatingProteinMass::solve(input).unwrap();
        assert!(output - 821.3919199999999 < f64::EPSILON);
    }
}
