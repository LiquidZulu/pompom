mod get_pomodoro;
use get_pomodoro::*;

mod types;
use types::*;

mod cli_args;
use cli_args::*;

mod do_cycle;
use do_cycle::*;

mod splash_screen;
use splash_screen::*;

mod config;
use config::*;

mod notify;
mod quotes;

use clap::Parser;

fn main() {
    let args = Args::parse();
    let config = get_config();

    let (work_duration, rest_duration, long_rest_duration) = get_pomodoro(args);

    splash_screen(&config.splash_screen_variant);

    do_multiple_cycles(
        config
            .schedule
            .iter()
            .map(|x| match x {
                Period::Work => (&work_duration, Period::Work),
                Period::Rest => (&rest_duration, Period::Rest),
                Period::LongRest => (&long_rest_duration, Period::LongRest),
            })
            .collect(),
    )
}
