use std::{
    fmt::Display,
    marker::PhantomData,
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use num::Integer;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Comma;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Whitespace;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Input parsed to an empty list")]
    EmptyList,

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DelimitedIntList<T, Del>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    values: Vec<T>,
    _delimeter: PhantomData<Del>,
}

impl<T, Del> Deref for DelimitedIntList<T, Del>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T, Del> DerefMut for DelimitedIntList<T, Del>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl<T, Del> From<Vec<T>> for DelimitedIntList<T, Del>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    fn from(v: Vec<T>) -> Self {
        Self {
            values: v,
            _delimeter: PhantomData,
        }
    }
}

impl<T, Del> Display for DelimitedIntList<T, Del>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
            .fmt(f)
    }
}

impl<T> FromStr for DelimitedIntList<T, Whitespace>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(T::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        if values.is_empty() {
            return Err(Error::EmptyList);
        }

        Ok(values.into())
    }
}

impl<T> FromStr for DelimitedIntList<T, Comma>
where
    T: Integer + FromStr<Err = ParseIntError> + Copy + Display,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .map(|v| T::from_str(v.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        if values.is_empty() {
            return Err(Error::EmptyList);
        }

        Ok(values.into())
    }
}

// convenience wrappers

/// Constructed from a whitespace-delimited list of i32 values.
pub type I32List = DelimitedIntList<i32, Whitespace>;

/// Constructed from a whitespace-delimited list of i64 values.
pub type I64List = DelimitedIntList<i64, Whitespace>;

/// Constructed from a whitespace-delimited list of u32 values.
pub type U32List = DelimitedIntList<u32, Whitespace>;

/// Constructed from a whitespace-delimited list of u64 values.
pub type U64List = DelimitedIntList<u64, Whitespace>;

/// Constructed from a whitespace-delimited list of usize values.
pub type UsizeList = DelimitedIntList<usize, Whitespace>;

#[cfg(test)]
mod tests {
    mod int_ist {
        use super::super::*;

        #[test]
        fn construction() {
            let obj = I32List::from_str("1 -2   777  4 5").unwrap();
            assert_eq!(obj, vec![1, -2, 777, 4, 5].into());
        }

        #[test]
        fn requires_valid_integers() {
            let res = U32List::from_str("1 -2   777  4 5");
            assert!(res.is_err());
        }

        #[test]
        fn erros_on_empty() {
            let res = U32List::from_str("            ");
            assert_eq!(res, Err(Error::EmptyList));
        }

        #[test]
        fn display() {
            let obj = I32List::from_str("1 -2   777  4 5").unwrap();
            assert_eq!(obj.to_string(), "1 -2 777 4 5".to_string());
        }
    }
}
