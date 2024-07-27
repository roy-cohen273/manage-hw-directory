use std::{fs, io};
use serde::Deserialize;

mod files;
mod interface;
mod config;

use interface::Interface;
use config::Config;
use interface::CmdInterface;

const CONFIG_FILE: &str = "config.toml";

fn main() -> io::Result<()> {
    let config_str = fs::read_to_string(CONFIG_FILE)?;
    let deserializer = toml::Deserializer::new(&config_str);
    let config = Config::deserialize(deserializer).map_err(io::Error::other)?;

    match CmdInterface::main(&config) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("ERROR: {e}");
            return Err(e);
        }
    }

    Ok(())
}
