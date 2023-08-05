use crate::quotes::*;
use crate::types::*;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use notify_rust::{Hint, Notification, Timeout};
use std::{path::Path, thread, time};

// I really don't give a hoot if this thing errors, it just wont play the sound.
fn play_sound(filename: &str) {
    match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
        Ok(mut manager) => {
            match StaticSoundData::from_file(filename, StaticSoundSettings::default()) {
                Ok(sound_data) => match manager.play(sound_data.clone()) {
                    _ => thread::sleep(time::Duration::from_secs(5)),
                },
                Err(error) => println!("{}", error),
            }
        }
        Err(_) => (),
    }
}

pub fn notify(period: &Period) {
    let maybe_data_dir = directories::ProjectDirs::from("com", "LiquidZulu", "pompom");
    match period {
        Period::Work => {
            Notification::new()
                .summary("Time to start working")
                .body(get_quote(QuoteTypes::Work))
                .icon("pompom")
                .appname("pompom")
                .hint(Hint::Category("Pomodoro".to_owned()))
                .timeout(Timeout::Milliseconds(10000))
                .show()
                .unwrap();
            thread::spawn(|| match maybe_data_dir {
                Some(data_dir) => play_sound(
                    (Path::new(data_dir.data_dir()).join("audio/1.mp3"))
                        .to_str()
                        .unwrap(),
                ),
                None => (),
            });
        }
        Period::Rest | Period::LongRest => {
            Notification::new()
                .summary("Time for a break")
                .body(get_quote(QuoteTypes::Rest))
                .icon("pompom")
                .appname("pompom")
                .hint(Hint::Category("Pomodoro".to_owned()))
                .timeout(Timeout::Milliseconds(10000))
                .show()
                .unwrap();
            thread::spawn(|| match maybe_data_dir {
                Some(data_dir) => play_sound(
                    (Path::new(data_dir.data_dir()).join("audio/1.mp3"))
                        .to_str()
                        .unwrap(),
                ),
                None => (),
            });
        }
    }
}
