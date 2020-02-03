use crate::ErrReport;
use crate::Result;
use std::fmt;

pub trait ContextObj: fmt::Display + Send + Sync + 'static {}

impl<T> ContextObj for T where T: fmt::Display + Send + Sync + 'static {}

pub trait ContextExt<T, E>: private::Sealed {
    /// Wrap the error value with additional context.
    fn note<C>(self, context: C) -> Result<T>
    where
        C: ContextObj;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_note<C, F>(self, f: F) -> Result<T>
    where
        C: ContextObj,
        F: FnOnce() -> C;
}

pub enum ContextObject {
    Note(Box<dyn ContextObj>),
    Warn(Box<dyn ContextObj>),
}

mod ext {
    use super::*;

    pub trait StdError {
        fn ext_context(self, context: ContextObject) -> ErrReport;
    }

    impl<E> StdError for E
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        fn ext_context(self, context: ContextObject) -> ErrReport {
            let mut error = ErrReport::from(self);
            error.0.context.push(context);
            error
        }
    }

    impl StdError for ErrReport {
        fn ext_context(mut self, context: ContextObject) -> ErrReport {
            self.0.context.push(context);
            self
        }
    }
}

impl<T, E> ContextExt<T, E> for std::result::Result<T, E>
where
    E: ext::StdError + Send + Sync + 'static,
{
    fn note<C>(self, context: C) -> Result<T>
    where
        C: ContextObj,
    {
        self.map_err(|error| error.ext_context(ContextObject::Note(Box::new(context))))
    }

    fn with_note<C, F>(self, context: F) -> Result<T>
    where
        C: ContextObj,
        F: FnOnce() -> C,
    {
        self.map_err(|error| error.ext_context(ContextObject::Note(Box::new(context()))))
    }
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T, E> Sealed for std::result::Result<T, E> where E: ext::StdError {}
    impl<T> Sealed for Option<T> {}
}
