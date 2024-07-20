mod cmd_interface;

use std::error::Error;
pub use cmd_interface::CmdInterface;

pub trait Interface {
    type Error: Error;

    fn main() -> Result<(), Self::Error>;
}
