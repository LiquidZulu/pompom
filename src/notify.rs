use crate::quotes::*;
use crate::types::*;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use notify_rust::{Hint, Notification, Timeout};
use std::{thread, time};

// I really don't give a hoot if this thing errors, it just wont play the sound.
fn play_sound(filename: &str) {
    match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
        Ok(mut manager) => {
            match StaticSoundData::from_file(filename, StaticSoundSettings::default()) {
                Ok(sound_data) => match manager.play(sound_data.clone()) {
                    _ => thread::sleep(time::Duration::from_secs(5)),
                },
                Err(_) => (),
            }
        }
        Err(_) => (),
    }
}

pub fn notify(period: &Period) {
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
            thread::spawn(|| play_sound("audio/0.mp3"));
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
            thread::spawn(|| play_sound("audio/1.mp3"));
        }
    }
}
