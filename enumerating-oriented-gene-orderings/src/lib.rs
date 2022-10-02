use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use florist_core::Error;
use florist_plumbing::Problem;
use itertools::Itertools;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Output(Vec<Vec<i32>>);

impl Deref for Output {
    type Target = Vec<Vec<i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Output {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .iter()
            .map(|vals| vals.iter().join(" "))
            .sorted()
            .join("\n");
        write!(f, "{}\n{}", self.len(), out)
    }
}

pub struct EnumeratingOrientedGeneOrderings;

impl Problem for EnumeratingOrientedGeneOrderings {
    type Error = Error;
    type Input = u16;
    type Output = Output;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let vals: Vec<Vec<i32>> = (1_i32..=(input as i32))
            .permutations(input as usize)
            .flat_map(|perm| perm.iter().map(|v| [-v, *v]).multi_cartesian_product())
            .collect();

        Ok(Output(vals))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn example() {
        let raw = "2";
        let input = u16::from_str(raw).expect("Failed to parse input");
        let mut output = EnumeratingOrientedGeneOrderings::solve(input).expect("Failed to solve");
        let mut expected = Output(vec![
            vec![-1, -2],
            vec![-1, 2],
            vec![1, -2],
            vec![1, 2],
            vec![-2, -1],
            vec![-2, 1],
            vec![2, -1],
            vec![2, 1],
        ]);

        output.sort();
        expected.sort();

        assert_eq!(output, expected);
    }
}
