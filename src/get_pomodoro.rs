use crate::cli_args::*;
use crate::config::*;
use crate::types::*;

pub fn get_pomodoro(args: Args) -> (Duration, Duration, Duration) {
    let config = get_config();
    let duration_variant = match args.unit {
        Unit::Seconds => Duration::Seconds,
        Unit::Minutes => Duration::Minutes,
        Unit::Hours => Duration::Hours,
    };

    match (
        args.work_duration,
        args.rest_duration,
        args.long_rest_duration,
    ) {
        (Some(w), Some(r), Some(l)) => (
            duration_variant(w),
            duration_variant(r),
            duration_variant(l),
        ),
        (Some(w), Some(r), None) => (
            duration_variant(w),
            duration_variant(r),
            config.long_rest_duration,
        ),
        (Some(w), None, Some(l)) => (
            duration_variant(w),
            config.rest_duration,
            duration_variant(l),
        ),
        (Some(w), None, None) => (
            duration_variant(w),
            config.rest_duration,
            config.long_rest_duration,
        ),
        (None, Some(r), Some(l)) => (
            config.work_duration,
            duration_variant(r),
            duration_variant(l),
        ),
        (None, Some(r), None) => (
            config.work_duration,
            duration_variant(r),
            config.long_rest_duration,
        ),
        (None, None, Some(l)) => (
            config.work_duration,
            config.rest_duration,
            duration_variant(l),
        ),
        (None, None, None) => (
            config.work_duration,
            config.rest_duration,
            config.long_rest_duration,
        ),
    }
}
