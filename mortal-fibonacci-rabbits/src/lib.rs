use std::collections::VecDeque;

use anyhow::bail;
use florist_inputs::UsizeList;
use florist_plumbing::Problem;

pub struct MortalFibonacciRabbits;

impl Problem for MortalFibonacciRabbits {
    type Error = anyhow::Error;
    type Input = UsizeList;
    // seriously, they don't even call this out and it's just dumb luck that
    // you don't get an input that overflows 64 bits.
    type Output = u128;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.len() != 2 {
            bail!("Input was not exactly two values: {:?}", input);
        }

        let months = input[0];
        let lifespan = input[1];

        if lifespan < 1 {
            bail!("Lifespan is too short: {}", lifespan);
        }

        let mut births: VecDeque<u128> = VecDeque::with_capacity(lifespan);
        births.push_front(1);
        births.push_front(0);

        let mut mature_pop = 0_u128;

        for _ in 2..months {
            mature_pop += births[1];
            let next = mature_pop;

            if births.len() == lifespan {
                mature_pop -= births.pop_back().unwrap();
            }

            births.push_front(next);
        }

        Ok(births[0] + births[1] + mature_pop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn mortality() {
        let input = UsizeList::from_str("8 4").unwrap();
        let output = MortalFibonacciRabbits::solve(input).unwrap();
        assert_eq!(output, 13);
    }
}
