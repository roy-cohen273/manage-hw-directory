mod cmd_interface;
mod tui_interface;

use crate::settings::Settings;
use cmd_interface::CmdInterface;
use serde::{Deserialize, Serialize};
use tui_interface::TuiInterface;

pub trait Interface {
    fn main(settings: &Settings) -> anyhow::Result<()>;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "UPPERCASE")]
pub enum InterfaceType {
    Cmd,
    Tui,
}

impl InterfaceType {
    pub fn main(&self, settings: &Settings) -> anyhow::Result<()> {
        match self {
            InterfaceType::Cmd => CmdInterface::main(settings),
            InterfaceType::Tui => TuiInterface::main(settings),
        }
    }
}
