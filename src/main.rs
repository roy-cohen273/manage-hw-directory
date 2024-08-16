use config::File;
use interface::CmdInterface;
use interface::Interface;
use settings::Settings;

mod files;
mod interface;
mod settings;

const SETTINGS_FILE: &str = "settings.json5";

fn main() -> anyhow::Result<()> {
    let settings = Settings::new([File::with_name(SETTINGS_FILE)])?;

    match CmdInterface::main(&settings) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("ERROR: {e}");
            anyhow::bail!(e);
        }
    }

    Ok(())
}
