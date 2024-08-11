use serde::Deserialize;
use std::marker::PhantomData;

pub trait Formattable {
    type Params: ?Sized;

    fn format(s: String, params: &Self::Params) -> anyhow::Result<String>;
}

#[derive(Deserialize)]
pub struct OpenConfig<T: Formattable> {
    binary: Box<str>,
    args_format: Box<[Box<str>]>,

    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T: Formattable> OpenConfig<T> {
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
