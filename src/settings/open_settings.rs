use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait Formattable {
    type Params: ?Sized;

    fn format(s: String, params: &Self::Params) -> anyhow::Result<String>;
}

#[derive(Serialize, Deserialize)]
pub struct OpenSettings<T: Formattable> {
    binary: Box<str>,
    args_format: Box<[Box<str>]>,

    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T: Formattable> Clone for OpenSettings<T> {
    fn clone(&self) -> Self {
        Self {
            binary: self.binary.clone(),
            args_format: self.args_format.clone(),
            _phantom: self._phantom.clone(),
        }
    }
}

impl<T: Formattable> OpenSettings<T> {
    pub fn binary(&self) -> &str {
        &self.binary
    }

    pub fn args(&self, params: &T::Params) -> anyhow::Result<impl Iterator<Item = String>> {
        self.args_format
            .iter()
            .map(|arg_format| T::format(arg_format.to_string(), params))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
            .map(IntoIterator::into_iter)
    }
}
