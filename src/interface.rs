mod cmd_interface;

use crate::settings::Settings;
pub use cmd_interface::CmdInterface;

pub trait Interface {
    fn main(settings: &Settings) -> anyhow::Result<()>;
}
