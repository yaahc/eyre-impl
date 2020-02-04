use crate::chain::Chain;
#[cfg(backtrace)]
use std::backtrace::Backtrace;
use std::fmt;

pub trait ErrorFormatter {
    fn fmt_error<'a>(
        &self,
        error: &'a (dyn std::error::Error + 'static),
        f: &mut fmt::Formatter,
    ) -> fmt::Result;
}

pub trait ErrorContext<CO> {
    fn push(&mut self, context: CO);
}

pub struct ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
{
    error: E,
    pub context: C,
}

impl<C, E> From<E> for ErrorReporter<E, C>
where
    C: Default,
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self {
            context: Default::default(),
            error,
        }
    }
}

/// CO = ContextObject
pub trait StdError<E, C, CO>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ext_context(self, context: CO) -> ErrorReporter<E, C>;
}

impl<E, C, CO> StdError<E, C, CO> for E
where
    C: Default + ErrorContext<CO>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn ext_context(self, context: CO) -> ErrorReporter<E, C> {
        let mut error = ErrorReporter::<E, C>::from(self);
        error.context.push(context);
        error
    }
}

impl<C, CO, E> StdError<E, C, CO> for ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
    C: ErrorContext<CO>,
{
    fn ext_context(mut self, context: CO) -> ErrorReporter<E, C> {
        self.context.push(context);
        self
    }
}

impl<E, C> ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        &self.error
    }

    pub fn chain(&self) -> Chain {
        Chain::new(self.error())
    }
}

impl<E, C> fmt::Display for ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
    C: Default + ErrorFormatter,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error())?;

        if f.alternate() {
            for cause in self.chain().skip(1) {
                write!(f, ": {}", cause)?;
            }
        }

        Ok(())
    }
}

impl<E, C> fmt::Debug for ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
    C: Default + ErrorFormatter,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = self.error();

        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        self.context.fmt_error(error, f)
    }
}

pub struct Indented<'a, D> {
    inner: &'a mut D,
    ind: Option<usize>,
    started: bool,
}

impl<'a, D> Indented<'a, D> {
    pub fn numbered(inner: &'a mut D, ind: usize) -> Self {
        Self {
            inner,
            ind: Some(ind),
            started: false,
        }
    }
}

impl<T> fmt::Write for Indented<'_, T>
where
    T: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (ind, mut line) in s.split('\n').enumerate() {
            if !self.started {
                // trim first line to ensure it lines up with the number nicely
                line = line.trim();
                // Don't render the first line unless its actually got text on it
                if line.is_empty() {
                    continue;
                }

                self.started = true;
                match self.ind {
                    Some(ind) => self.inner.write_fmt(format_args!("{: >5}: ", ind))?,
                    None => self.inner.write_fmt(format_args!("    "))?,
                }
            } else if ind > 0 {
                self.inner.write_char('\n')?;
                if self.ind.is_some() {
                    self.inner.write_fmt(format_args!("       "))?;
                } else {
                    self.inner.write_fmt(format_args!("    "))?;
                }
            }

            self.inner.write_fmt(format_args!("{}", line))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write as _;

    #[test]
    fn one_digit() {
        let input = "verify\nthis";
        let expected = "    2: verify\n       this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            ind: Some(2),
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn two_digits() {
        let input = "verify\nthis";
        let expected = "   12: verify\n       this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            ind: Some(12),
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn no_digits() {
        let input = "verify\nthis";
        let expected = "    verify\n    this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            ind: None,
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }
}
