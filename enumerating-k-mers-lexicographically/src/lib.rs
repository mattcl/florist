use anyhow::bail;
use florist_plumbing::Problem;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Input {
    symbols: Vec<char>,
    n: usize,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("Cannot parse symbols from empty input");
        }

        let lines: Vec<_> = s.lines().collect();

        if lines.len() != 2 {
            bail!("Invalid number of lines: {}", s)
        }

        let symbols = lines[0].chars().filter(|ch| !ch.is_whitespace()).collect();
        let n = lines[1].parse()?;

        Ok(Self { symbols, n })
    }
}

pub struct EnumeratingKMersLexicographically;

impl Problem for EnumeratingKMersLexicographically {
    type Error = anyhow::Error;
    type Input = Input;
    type Output = String;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input
            .symbols
            .iter()
            .combinations_with_replacement(input.n)
            // brute force, maybe i'll make it faster laster, but the inputs are
            // so small
            .flat_map(|comb| comb.into_iter().permutations(input.n).unique())
            .map(String::from_iter)
            .sorted()
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerating() {
        let raw = "A C G T
2";
        let input = Input::from_str(raw).unwrap();
        let output = EnumeratingKMersLexicographically::solve(input).unwrap();
        let expected = "AA
AC
AG
AT
CA
CC
CG
CT
GA
GC
GG
GT
TA
TC
TG
TT";
        assert_eq!(output, expected);
    }
}
