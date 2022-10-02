use florist_core::Error;
use florist_plumbing::Problem;

pub struct {{project-name|upper_camel_case}};

impl Problem for {{project-name|upper_camel_case}} {
    type Error = Error;
    type Input = usize;
    type Output = usize;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn example() {
        let raw = "10";
        let input = usize::from_str(raw).expect("Failed to parse input");
        let output = {{project-name|upper_camel_case}}::solve(input).expect("Failed to solve");
        assert_eq!(output, 10);
    }
}
