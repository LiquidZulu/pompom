use crate::splash_screen::*;
use crate::types::*;
use colored::Colorize;
use directories;
use serde::Serialize;
use std::{fs, io::Write, path::Path};
use toml::Table;

#[derive(Serialize)]
pub struct Config {
    pub work_duration: Duration,
    pub rest_duration: Duration,
    pub long_rest_duration: Duration,
    pub splash_screen_variant: SplashScreen,
    pub schedule: Vec<Period>,
}

pub fn get_config() -> Config {
    let default_config: Config = Config {
        work_duration: Duration::Minutes(25),
        rest_duration: Duration::Minutes(5),
        long_rest_duration: Duration::Minutes(30),
        splash_screen_variant: SplashScreen::Row,
        schedule: vec![
            Period::Work,
            Period::Rest,
            Period::Work,
            Period::Rest,
            Period::Work,
            Period::Rest,
            Period::Work,
            Period::LongRest,
        ],
    };

    match directories::ProjectDirs::from("com", "LiquidZulu", "pompom") {
        Some(dirs) => {
            let config_path = Path::new(dirs.config_dir());
            let config_file = config_path.join("config.toml");

            if !config_path.exists() {
                match fs::create_dir_all(config_path) {
                    Ok(_) => (),
                    Err(e) => {
                        println!(
                            "{}: Unable to create config dir {}\n\tUsing default config.\n\n{}",
                            "WARNING".yellow(),
                            config_path.display(),
                            e
                        );
                        return default_config;
                    }
                };
            };

            if !config_file.exists() {
                return match fs::File::create(config_file) {
                    Ok(mut new_config_file) => match toml::to_string(&default_config) {
                        Ok(config_toml_string) => {
                            match write!(&mut new_config_file, "{}", config_toml_string) {
                                Ok(_) => println!(
                                    "{}: new config file created at {:?}",
                                    "INFO".blue(),
                                    config_path
                                ),
                                Err(e) => println!(
                                    "{}: unable to create config file at {:?}\n{}",
                                    "WARNING".yellow(),
                                    config_path,
                                    e
                                ),
                            };
                            default_config
                        }
                        Err(e) => {
                            println!("{}: {}", "ERROR".red(), e);
                            default_config
                        }
                    },
                    Err(e) => {
                        println!("{}: {}", "ERROR".red(), e);
                        default_config
                    }
                };
            };

            match fs::read_to_string(config_file.clone()) {
                Ok(config_file_contents) => match config_file_contents.parse::<Table>() {
                    Ok(table) => {
                        return Config {
                            schedule: match table["schedule"].as_array() {
                                Some(schedule) => schedule
                                    .iter()
                                    .map(|x| match x.as_str() {
                                        Some(y) => match y.parse::<Period>() {
                                            Ok(parsed) => Some(parsed),
                                            Err(()) => None,
                                        },
                                        None => None,
                                    })
                                    .filter(|x| match x {
                                        None => false,
                                        Some(_) => true,
                                    })
                                    .map(|x| x.unwrap())
                                    .collect(),
                                None => default_config.schedule,
                            },
                            work_duration: {
                                match (
                                    table["work_duration"].get("Seconds"),
                                    table["work_duration"].get("Minutes"),
                                    table["work_duration"].get("Hours"),
                                ) {
                                    (Some(secs), _, _) => match secs.as_integer() {
                                        Some(value) => Duration::Seconds(value.abs() as u64),
                                        None => default_config.work_duration,
                                    },
                                    (_, Some(mins), _) => match mins.as_integer() {
                                        Some(value) => Duration::Minutes(value.abs() as u64),
                                        None => default_config.work_duration,
                                    },
                                    (_, _, Some(hours)) => match hours.as_integer() {
                                        Some(value) => Duration::Hours(value.abs() as u64),
                                        None => default_config.work_duration,
                                    },
                                    (None, None, None) => {
                                        println!(
                                            "{}: could not parse work_duration",
                                            "ERROR".red()
                                        );
                                        default_config.work_duration
                                    }
                                }
                            },
                            rest_duration: {
                                match (
                                    table["rest_duration"].get("Seconds"),
                                    table["rest_duration"].get("Minutes"),
                                    table["rest_duration"].get("Hours"),
                                ) {
                                    (Some(secs), _, _) => match secs.as_integer() {
                                        Some(value) => Duration::Seconds(value.abs() as u64),
                                        None => default_config.rest_duration,
                                    },
                                    (_, Some(mins), _) => match mins.as_integer() {
                                        Some(value) => Duration::Minutes(value.abs() as u64),
                                        None => default_config.rest_duration,
                                    },
                                    (_, _, Some(hours)) => match hours.as_integer() {
                                        Some(value) => Duration::Hours(value.abs() as u64),
                                        None => default_config.rest_duration,
                                    },
                                    (None, None, None) => {
                                        println!(
                                            "{}: could not parse rest_duration",
                                            "ERROR".red()
                                        );
                                        default_config.rest_duration
                                    }
                                }
                            },
                            long_rest_duration: {
                                match (
                                    table["long_rest_duration"].get("Seconds"),
                                    table["long_rest_duration"].get("Minutes"),
                                    table["long_rest_duration"].get("Hours"),
                                ) {
                                    (Some(secs), _, _) => match secs.as_integer() {
                                        Some(value) => Duration::Seconds(value.abs() as u64),
                                        None => default_config.long_rest_duration,
                                    },
                                    (_, Some(mins), _) => match mins.as_integer() {
                                        Some(value) => Duration::Minutes(value.abs() as u64),
                                        None => default_config.long_rest_duration,
                                    },
                                    (_, _, Some(hours)) => match hours.as_integer() {
                                        Some(value) => Duration::Hours(value.abs() as u64),
                                        None => default_config.long_rest_duration,
                                    },
                                    (None, None, None) => {
                                        println!(
                                            "{}: could not parse long_rest_duration",
                                            "ERROR".red()
                                        );
                                        default_config.long_rest_duration
                                    }
                                }
                            },
                            splash_screen_variant: match table["splash_screen_variant"].as_str() {
                                Some(body) => match body.parse::<SplashScreen>() {
                                    Ok(once) => once,
                                    Err(_) => default_config.splash_screen_variant,
                                },
                                None => default_config.splash_screen_variant,
                            },
                        };
                    }
                    Err(e) => {
                        println!(
                            "{} at parsing contents of {} as Table: {}",
                            "ERROR".red(),
                            config_file.display(),
                            e
                        );
                        default_config
                    }
                },
                Err(e) => {
                    println!(
                        "{} at reading contents of {}: {}",
                        "ERROR".red(),
                        config_file.display(),
                        e
                    );
                    default_config
                }
            }
        }
        None => {
            println!(
                "{}: Unable to get project dirs at config.rs\n\tUsing default config.",
                "WARNING".yellow()
            );
            default_config
        }
    }
}
