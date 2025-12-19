use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub modpack_dir: PathBuf,
    pub state_file: PathBuf,
    pub minecraft_mod_dir: Option<PathBuf>,
}

impl Configuration {
    fn default(base: &PathBuf) -> Configuration {
        return Configuration {
            modpack_dir: base.join("modpacks"),
            state_file: base.join(".state"),
            minecraft_mod_dir: get_minecraft_dir(),
        };
    }
}

pub fn get_config() -> Result<Configuration, ()> {
    let Ok(base) = get_base_dir() else {
        return Err(());
    };

    let conf_path = base.join("config.toml");

    if !conf_path.exists() {
        // Make a new configuration file
        let default_configuration = Configuration::default(&base);
        save_config(&default_configuration);
        return Ok(default_configuration);
    }

    let conf_file_contents = match fs::read_to_string(conf_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading config.toml: ({err})");
            return Err(());
        }
    };

    match toml::from_str(&conf_file_contents) {
        Ok(contents) => return Ok(contents),
        Err(err) => {
            eprintln!("Error deserialising config.toml ({err})");
            return Err(());
        }
    };
}

fn save_config(conf: &Configuration) {
    let Ok(conf_as_toml) = toml::to_string(conf) else {
        eprintln!(
            "Could not serialise config. If you are reading this, something very weird has happened"
        );
        return;
    };

    let Ok(base) = get_base_dir() else {
        return;
    };
    let conf_path = base.join("config.toml");

    let Ok(mut file) = File::create(conf_path) else {
        eprintln!("Could not create new config.toml");
        return;
    };

    match file.write_all(conf_as_toml.as_bytes()) {
        Ok(_) => (),
        Err(err) => eprintln!("Could not save to config.toml ({err})"),
    }
}

fn get_base_dir() -> Result<PathBuf, ()> {
    let Some(home) = env::home_dir() else {
        eprintln!("Could not find user home directory");
        return Err(());
    };

    let base = home.join(Path::new(".mpreg"));
    Ok(base)
}

fn get_minecraft_dir() -> Option<PathBuf> {
    let Some(home) = env::home_dir() else {
        eprintln!(
            "Could not find user home directory to find .minecraft folder.
Try defining minecraft_mod_dir in config.toml"
        );
        return None;
    };

    let minecraft_mod_dir = home.join(Path::new(".minecraft/mods"));
    if !minecraft_mod_dir.exists() {
        eprintln!(
            "Could not find mod directory.
Try defining minecraft_mod_dir in config.toml"
        );
        return None;
    }

    return Some(minecraft_mod_dir);
}
