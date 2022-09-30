use anyhow::bail;
use florist_inputs::U32List;
use florist_plumbing::Problem;

pub struct IndependentAlleles;

impl Problem for IndependentAlleles {
    type Error = anyhow::Error;
    type Input = U32List;
    type Output = f64;

    fn solve(input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.len() != 2 {
            bail!("Invalid input, wrong number of values: {}", input);
        }

        let k = input[0];
        let n = input[1];

        let base = 2_u32;
        let population = base.pow(k);

        if population == n {
            return Ok(0.25_f64.powi(n as i32));
        } else if n == 0 {
            return Ok(0.75_f64.powi(population as i32));
        }

        Ok(if n < population / 2 {
            let other = population - n + 1;
            1.0 - (other..=population)
                .map(|k| binomial(k, population, 0.75))
                .sum::<f64>()
        } else {
            (n..=population)
                .map(|k| binomial(k, population, 0.25))
                .sum()
        })
    }
}

fn binomial(k: u32, n: u32, p: f64) -> f64 {
    binomial_coeff(k, n) * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}

fn binomial_coeff(k: u32, n: u32) -> f64 {
    (1..=k).map(|i| (n + 1 - i) as f64 / i as f64).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn expected_genotypes() {
        let input = U32List::from_str("2 1").unwrap();
        let output = IndependentAlleles::solve(input).unwrap();
        assert!((output - 0.68359375).abs() < f64::EPSILON);
    }
}
