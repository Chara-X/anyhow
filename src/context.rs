use super::*;
use std::fmt;
/// [anyhow::Context]
pub trait Context<T, E>: Sealed {
    /// [anyhow::Context::context]
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static;
    /// [anyhow::Context::with_context]
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}
impl<T> Context<T, Error> for Result<T, Error> {
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.context(context)),
        }
    }
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.context(f())),
        }
    }
}
impl<T, E> Sealed for Result<T, E> {}
