use std::{
    env,
    path::{Path, PathBuf},
};

pub struct Configuration {
    pub base_dir: PathBuf,
    pub modpack_dir: PathBuf,
}

pub fn get_config() -> Result<Configuration, ()> {
    let home = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("User home directory could not be determined");
            return Err(());
        }
    };

    let base: PathBuf = home.join(Path::new(".mpreg"));

    return Ok(Configuration {
        base_dir: base.clone(),
        modpack_dir: base.clone().join(Path::new("modpacks")),
    });
}
