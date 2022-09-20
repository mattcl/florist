use std::{fmt::Display, io, path::Path, str::FromStr};

/// Indicates the following structure represents a problem solution.
pub trait Problem {
    /// The error type when solving this problem.
    type Error;

    /// The input type when solving this problem.
    type Input: FromStr;

    /// The output type representing the solution to this problem.
    type Output: Display;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error>;

    /// Conveninece function leveraged by the solver to make inputs.
    fn input_from(val: &str) -> Result<Self::Input, <Self::Input as FromStr>::Err> {
        Self::Input::from_str(val)
    }
}

pub fn load_input_file(name: &str) -> Result<String, io::Error> {
    let path = Path::new("../inputs").join(name);
    std::fs::read_to_string(path).map(|s| s.trim().to_string())
}
