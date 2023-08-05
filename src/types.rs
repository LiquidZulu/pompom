use clap::ValueEnum;
use serde::Serialize;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
}

#[derive(Debug, Serialize)]
pub enum Duration {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
}

#[derive(Debug, Serialize)]
pub enum Period {
    Work,
    Rest,
    LongRest,
}

impl FromStr for Period {
    type Err = ();

    fn from_str(s: &str) -> Result<Period, ()> {
        match s {
            "Work" => Ok(Period::Work),
            "Rest" => Ok(Period::Rest),
            "LongRest" => Ok(Period::LongRest),
            _ => Err(()),
        }
    }
}
