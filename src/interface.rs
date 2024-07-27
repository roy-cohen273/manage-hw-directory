mod cmd_interface;

use std::error::Error;
use crate::config::Config;
pub use cmd_interface::CmdInterface;

pub trait Interface {
    type Error: Error;

    fn main(config: &Config) -> Result<(), Self::Error>;
}
