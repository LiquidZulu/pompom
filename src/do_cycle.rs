use crate::notify::*;
use crate::quotes::*;
use crate::types::*;
use colored::Colorize;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::{thread, time};

fn get_duration_values(duration: &Duration) -> (u64, u64) {
    let duration_int = match duration {
        Duration::Seconds(x) => x,
        Duration::Minutes(x) => x,
        Duration::Hours(x) => x,
    };

    let mins = duration_int * 60;
    let hours = duration_int * 60 * 60;
    let duration_normalised = match duration {
        Duration::Seconds(_) => duration_int,
        Duration::Minutes(_) => &mins,
        Duration::Hours(_) => &hours,
    };

    (*duration_int, *duration_normalised)
}

fn get_bar_style<'a>(color: &'a str, duration: &'a Duration, period: &'a Period) -> String {
    let (duration_int, _duration_normalised) = get_duration_values(duration);
    format!(
        "{} [{{bar:40.{}}}] {{elapsed}} / {} ({{percent}}%) {{msg}}",
        match period {
            Period::Work => "WORK".cyan(),
            Period::Rest => "REST".magenta(),
            Period::LongRest => "LONG REST".green(),
        },
        color,
        match duration {
            Duration::Seconds(_) =>
                HumanDuration(time::Duration::from_secs(duration_int)).to_string(),
            Duration::Minutes(_) =>
                HumanDuration(time::Duration::from_secs(duration_int) * 60).to_string(),
            Duration::Hours(_) =>
                HumanDuration(time::Duration::from_secs(duration_int) * 60 * 60).to_string(),
        },
    )
}

pub fn do_cycle(duration: &Duration, period: Period) {
    notify(&period);
    let (_duration_int, duration_normalised) = get_duration_values(&duration);

    let bar = ProgressBar::new(duration_normalised);

    match period {
        Period::Work => bar.set_style(
            ProgressStyle::with_template(&get_bar_style("cyan", &duration, &period))
                .unwrap()
                .progress_chars("##-"),
        ),
        Period::Rest => bar.set_style(
            ProgressStyle::with_template(&get_bar_style("magenta", &duration, &period))
                .unwrap()
                .progress_chars("##-"),
        ),
        Period::LongRest => bar.set_style(
            ProgressStyle::with_template(&get_bar_style("green", &duration, &period))
                .unwrap()
                .progress_chars("##-"),
        ),
    };

    for i in 0..duration_normalised {
        bar.inc(1);
        if i % 30 == 0 {
            bar.set_message(get_quote(match period {
                Period::Work => QuoteTypes::Work,
                Period::Rest => QuoteTypes::Rest,
                Period::LongRest => QuoteTypes::Rest,
            }))
        }
        thread::sleep(time::Duration::from_secs(1));
    }
    bar.finish();
}

pub fn do_multiple_cycles(cycle_chain: Vec<(&Duration, Period)>) {
    for cycle in cycle_chain {
        let (duration, period) = cycle;
        do_cycle(duration, period);
    }
}
