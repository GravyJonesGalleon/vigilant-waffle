use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::conf_io::Configuration;

#[derive(Serialize, Deserialize, Debug)]
pub struct MpregState {
    pub current_modpack: Option<PathBuf>,
}

impl MpregState {
    fn default() -> MpregState {
        return MpregState {
            current_modpack: None,
        };
    }
}

pub fn get_state(conf: &Configuration) -> Result<MpregState, ()> {
    if !conf.state_file.exists() {
        // Make a new configuration file
        let default_state = MpregState::default();
        save_state(&default_state, conf);
        return Ok(default_state);
    }

    let state_file_contents = match fs::read_to_string(&conf.state_file) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!(
                "Error reading {}: ({err})",
                conf.state_file.as_os_str().display()
            );
            return Err(());
        }
    };

    match toml::from_str(&state_file_contents) {
        Ok(contents) => return Ok(contents),
        Err(err) => {
            eprintln!(
                "Error deserialising {} ({err})",
                conf.state_file.as_os_str().display()
            );
            return Err(());
        }
    };
}

fn save_state(state: &MpregState, conf: &Configuration) {
    let Ok(state_as_toml) = toml::to_string(state) else {
        eprintln!(
            "Could not serialise state. If you are reading this, something very weird has happened"
        );
        return;
    };

    let Ok(mut file) = File::create(&conf.state_file) else {
        eprintln!(
            "Could not create new {}",
            conf.state_file.as_os_str().display()
        );
        return;
    };

    match file.write_all(state_as_toml.as_bytes()) {
        Ok(_) => (),
        Err(err) => eprintln!(
            "Could not save to {} ({err})",
            conf.state_file.as_os_str().display()
        ),
    }
}
