use clap::{Args, Subcommand, command};
use std::fs::{self, DirBuilder};
use std::path::PathBuf;

use crate::conf_io::Configuration;
use crate::state_io::MpregState;

#[derive(Debug, Args)]
pub struct PackArgs {
    #[command(subcommand)]
    command: Option<PackCommands>,
}

#[derive(Debug, Subcommand)]
enum PackCommands {
    /// Create a new mod pack
    #[command(arg_required_else_help = true)]
    Create {
        /// Name of the mod pack
        name: String,
    },
    /// List all existing mod packs
    List,
    /// Delete a mod pack
    #[command(arg_required_else_help = true)]
    Delete {
        /// The mod pack to remove
        target: String,
    },
    /// Rename a mod pack
    #[command(arg_required_else_help = true)]
    Rename {
        /// The mod pack to rename
        old: String,
        /// The new name
        new: String,
    },
    /// Switch to a different mod pack
    #[command(arg_required_else_help = true)]
    Switch {
        /// The mod pack to switch to
        target: String,
    },
}

pub fn main(args: PackArgs, conf: &Configuration, state: &MpregState) {
    match args.command {
        Some(PackCommands::Create { name }) => create(name, conf),
        Some(PackCommands::List) => list(conf),
        Some(PackCommands::Delete { target }) => delete(target, conf),
        Some(PackCommands::Rename { old, new }) => rename(old, new, conf),
        Some(PackCommands::Switch { target }) => switch(target),
        None => show(state),
    }
}

fn create(name: String, conf: &Configuration) {
    let path = conf.modpack_dir.join(&name);

    if path.exists() {
        println!("WARNNG: Modpack called {name} already exists");
        return;
    }

    match DirBuilder::new().recursive(true).create(path) {
        Ok(_) => {
            println!("OK: Created new modpack called {name}");
            return;
        }
        Err(err) => {
            eprintln!("ERROR: Error occurred ({err})");
            return;
        }
    }
}

fn list(conf: &Configuration) {
    let modpack_dir = &conf.modpack_dir;
    let Ok(modpack_dir_entries) = modpack_dir.read_dir() else {
        eprintln!("ERROR: Could not read the modpack directory!");
        return;
    };
    for entry in modpack_dir_entries {
        if let Ok(entry) = entry {
            let entry = entry.file_name();
            println!("{}", entry.display())
        }
    }
}

fn delete(target: String, conf: &Configuration) {
    let modpack = conf.modpack_dir.join(&target);
    match try_delete_modpack(&modpack) {
        Ok(_) => println!("OK: {target} deleted"),
        Err(message) => eprintln!("ERROR: {message}"),
    }
}

fn rename(old: String, new: String, conf: &Configuration) {
    let old_path = conf.modpack_dir.join(&old);
    let new_path = conf.modpack_dir.join(&new);
    match try_rename_modpack(&old_path, &new_path) {
        Ok(_) => println!("OK: {old} renamed to {new}"),
        Err(message) => eprintln!("ERROR: {message}"),
    }
}

fn switch(target: String) {
    println!("Moving to {target}");
}

fn show(state: &MpregState) {
    match &state.current_modpack {
        None => println!("No modpack currently active"),
        Some(modpack) => {
            let name = match modpack.file_name() {
                Some(name) => name,
                None => modpack.as_os_str(),
            };
            println!("Current modpack: {}", name.display());
        }
    }
}

/// Determine if the provided path is a valid modpack
/// (directory in the modpacks folder containing only symlinks)
fn is_modpack(modpack: &PathBuf) -> bool {
    if !modpack.exists() {
        return false;
    }

    // A modpack should only contain symlinks
    let Ok(entries) = modpack.read_dir() else {
        eprintln!("ERROR: Could not access modpack");
        return false;
    };

    // Lots of unwrapping required here, but this returns false if any entry is not a symlink
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(entry) = entry.file_type() {
                if !entry.is_symlink() {
                    return false;
                }
            }
        }
    }
    return true;
}

fn try_delete_modpack(modpack: &PathBuf) -> Result<(), String> {
    if !is_modpack(&modpack) {
        return Err(String::from(format!(
            "File at {} could not be deleted because it is not a modpack
Modpacks must be directories containing only symlinks.",
            modpack.display()
        )));
    }
    match fs::remove_dir_all(modpack) {
        Err(err) => {
            return Err(String::from(format!(
                "File at {} could not be deleted ({err})",
                modpack.display()
            )));
        }
        Ok(_) => return Ok(()),
    }
}

fn try_rename_modpack(old: &PathBuf, new: &PathBuf) -> Result<(), String> {
    if !is_modpack(&old) {
        return Err(String::from(format!(
            "File at {} could not be renamed because it is not a modpack
Modpacks must be directories containing only symlinks.",
            old.display()
        )));
    }
    if new.exists() {
        return Err(String::from(format!(
            "Could not rename because {} already exists.",
            new.display()
        )));
    }

    match fs::rename(old, new) {
        Err(err) => {
            return Err(String::from(format!(
                "File at {} could not be renamed ({err})",
                old.display()
            )));
        }
        Ok(_) => return Ok(()),
    }
}
