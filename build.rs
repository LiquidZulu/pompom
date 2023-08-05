use directories;
use std::{fs, path::Path};

fn main() {
    match directories::ProjectDirs::from("com", "LiquidZulu", "pompom") {
        Some(thing) => {
            let data_dir = thing.data_dir();
            match fs::create_dir(data_dir) {
                Ok(_) => (),
                Err(_) => (),
            };

            match fs::create_dir(Path::new(data_dir).join("audio/")) {
                Ok(_) => (),
                Err(_) => (),
            };

            for file in ["audio/0.mp3", "audio/1.mp3"] {
                match fs::copy(
                    Path::new(&std::env::current_dir().unwrap()).join(file),
                    Path::new(data_dir).join(file),
                ) {
                    Ok(_) => (),
                    Err(error) => panic!("{}", error),
                };
            }
        }
        None => (),
    }
}
