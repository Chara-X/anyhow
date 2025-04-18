use std::{error, fmt, ops};
/// [anyhow::Error]
pub struct Error(Box<dyn error::Error + Send + Sync + 'static>);
impl Error {
    /// [anyhow::Error::new]
    pub fn new<E>(error: E) -> Self
    where
        E: error::Error + Send + Sync + 'static,
    {
        Error(Box::new(error))
    }
    /// [anyhow::Error::msg]
    pub fn msg<M>(message: M) -> Self
    where
        M: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        Error(Box::new(MessageError(message)))
    }
    /// [anyhow::Error::context]
    pub fn context<C>(self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        Error(Box::new(ContextError {
            error: self,
            context,
        }))
    }
    /// [anyhow::Error::chain]
    pub fn chain(&self) -> Chain<'_> {
        Chain {
            next: Some(self.0.as_ref()),
        }
    }
}
impl<E> From<E> for Error
where
    E: error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Error::new(value)
    }
}
impl ops::Deref for Error {
    type Target = dyn error::Error + Send + Sync;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
struct MessageError<M>(M);
impl<M> error::Error for MessageError<M> where M: fmt::Debug + fmt::Display {}
impl<M> fmt::Debug for MessageError<M>
where
    M: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<M> fmt::Display for MessageError<M>
where
    M: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
struct ContextError<C, E> {
    pub context: C,
    pub error: E,
}
impl<C, E> error::Error for ContextError<C, E>
where
    C: fmt::Display,
    E: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}
impl<C> error::Error for ContextError<C, Error>
where
    C: fmt::Display,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.error.0.as_ref())
    }
}
impl<C, E> fmt::Debug for ContextError<C, E>
where
    C: fmt::Display,
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.context.fmt(f)?;
        self.error.fmt(f)
    }
}
impl<C, E> fmt::Display for ContextError<C, E>
where
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.context.fmt(f)
    }
}
/// [anyhow::Chain]
pub struct Chain<'a> {
    next: Option<&'a (dyn error::Error + 'static)>,
}
impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn error::Error + 'static);
    fn next(&mut self) -> Option<Self::Item> {
        let error = self.next?;
        self.next = error.source();
        Some(error)
    }
}
