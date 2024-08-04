mod cmd_interface;

use crate::config::Config;
pub use cmd_interface::CmdInterface;

pub trait Interface {
    fn main(config: &Config) -> anyhow::Result<()>;
}
