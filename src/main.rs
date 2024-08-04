use std::fs;

use config::Config;
use interface::CmdInterface;
use interface::Interface;

mod config;
mod files;
mod interface;

const CONFIG_FILE: &str = "config.toml";

fn main() -> anyhow::Result<()> {
    let config_str = fs::read_to_string(CONFIG_FILE)?;
    let config: Config = toml::from_str(&config_str)?;

    match CmdInterface::main(&config) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("ERROR: {e}");
            anyhow::bail!(e);
        }
    }

    Ok(())
}
