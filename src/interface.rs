mod cmd_interface;

use crate::config::Config;
pub use cmd_interface::CmdInterface;
use std::error::Error;

pub trait Interface {
    type Error: Error;

    fn main(config: &Config) -> Result<(), Self::Error>;
}
