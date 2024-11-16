use crate::interface::InterfaceType;
use crate::subject::Subject;
use formatx::formatx;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceSettings {
    #[serde(rename = "type")]
    interface_type: InterfaceType,
    subject_label_format: Box<str>,
}

impl InterfaceSettings {
    pub fn interface_type(&self) -> &InterfaceType {
        &self.interface_type
    }

    pub fn subject_label(&self, subject: &Subject) -> Result<String, formatx::Error> {
        formatx!(
            self.subject_label_format.to_owned(),
            name = subject.name(),
            num = subject.current_hw_num()
        )
    }
}
