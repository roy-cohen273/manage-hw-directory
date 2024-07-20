use std::io;

mod files;
mod interface;

use interface::Interface;
use crate::interface::CmdInterface;

fn main() -> io::Result<()> {
    match CmdInterface::main() {
        Ok(()) => {},
        Err(e) => {
            eprintln!("ERROR: {e}");
            return Err(e);
        }
    }

    Ok(())
}
