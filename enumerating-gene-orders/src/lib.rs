use std::{fmt::Display, ops::{Deref, DerefMut}};
use itertools::{join, Itertools};

use florist_plumbing::Problem;


#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Permutation(Vec<u32>);

impl Deref for Permutation {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Permutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        join(&self.0, " ").fmt(f)
    }
}


#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Permutations(Vec<Permutation>);

impl Deref for Permutations {
    type Target = Vec<Permutation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Permutations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Permutations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.len(), join(&self.0, "\n"))
    }
}


pub struct EnumeratingGeneOrders;

impl Problem for EnumeratingGeneOrders {
    type Error = std::io::Error;
    type Input = u32;
    type Output = Permutations;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let raw = (1..=input)
            .permutations(input as usize)
            .map(|p| {
                Permutation(p)
            })
            .collect::<Vec<_>>();

        Ok(Permutations(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_permutations() {
        let input = 3;
        let output = EnumeratingGeneOrders::solve(input).unwrap();
        assert_eq!(output.len(), 6);
    }
}
