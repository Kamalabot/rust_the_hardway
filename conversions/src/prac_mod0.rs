#![allow(unused_imports)]
#![allow(warnings)]

use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
// conv between f64 to i32
// conv between Struct types
// with into() via From trait
// using try_from via TryFrom trait
// to_string(), from_strigs on premitive dataset

pub fn as_conv() {
    let int_val = 10;
    let float_val = int_val as f64;
    println!("converted {} to float {}", int_val, float_val)
}
//work on custom struct type
#[derive(Debug, Copy, Clone)]
pub struct Celsius(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Farenheit {
    pub value: f64,
}

impl From<Celsius> for Farenheit {
    fn from(value: Celsius) -> Farenheit {
        Farenheit {
            value: (value.0 * 9.0 / 5.0) + 32.0,
        }
    }
}

impl From<Farenheit> for Celsius {
    fn from(value: Farenheit) -> Self {
        Celsius((value.value - 32.0) * (5.0 / 9.0))
    }
}

impl Display for Farenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temperature in F: {}", &self.value)
    }
}
impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temperature in C: {}", &self.0)
    }
}

impl TryFrom<&str> for Celsius {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let temp = value.parse::<f64>()?;
        Ok(Celsius(temp))
    }
}

impl TryFrom<&str> for Farenheit {
    type Error = ParseFloatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ft = value.parse::<f64>()?;
        Ok(Farenheit { value: ft })
    }
}

impl Celsius {
    pub fn new(value: f64) -> Self {
        Celsius(value)
    }
}
impl Farenheit {
    pub fn new(value: f64) -> Self {
        Farenheit { value }
    }
}
