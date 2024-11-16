use config::File;
use settings::Settings;

mod interface;
mod settings;
mod subject;

const SETTINGS_FILE: &str = if cfg!(debug_assertions) {
    "example_settings.json5"
} else {
    "settings.json5"
};

fn main() -> anyhow::Result<()> {
    let settings = Settings::new([File::with_name(SETTINGS_FILE)])?;

    match settings
        .interface_settings()
        .interface_type()
        .main(&settings)
    {
        Ok(()) => {}
        Err(e) => {
            eprintln!("ERROR: {e}");
            anyhow::bail!(e);
        }
    }

    Ok(())
}
