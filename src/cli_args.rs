use crate::types::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub work_duration: Option<u64>,
    pub rest_duration: Option<u64>,
    pub long_rest_duration: Option<u64>,

    #[arg(short, long, value_enum, default_value_t = Unit::Minutes)]
    pub unit: Unit,
}
