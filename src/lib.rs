mod chain;

pub use chain::Chain;
use std::fmt;

/// Helper trait for inserting generic types representing context into a Context
pub trait ErrorContext<CO> {
    fn push(&mut self, context: CO);
}

/// A general purpose error reporting type for holding generic errors and associating generic
/// context with those errors
///
/// This type is intended to be consumed by a library that implements a concrete error type that is
/// designed for the specific use cases the user has. The library implementer should provide two
/// types, an error type of their choice, and a Context type that implements the two main helper
/// traits, `ErrorFormatter` and `ErrorContext`
pub struct ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
{
    pub error: E,
    pub context: C,
}

impl<E, C> ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// Construct an iterator over the internal error and its sources
    pub fn chain(&self) -> Chain {
        Chain::new(&self.error)
    }
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

/// Helper trait for creating error reporters from context in extension Traits
///
/// This trait is intended to be used by extension traits for Result so that you can write
/// combinators that work on either std::error::Error types directly (thus creating a new
/// ErrorReporter) or on types that own an ErrorReporter, which lets you implement error types with
/// APIs such as in the example
///
/// ## Example of supported APIs
///
/// ```rust
/// let val = fallible_fn()
///             .note("This function can only be used in certain situations")
///             .warn("Be careful not to use this fn in this specific situation")?;
/// ```
pub trait IntoErrorReporter<E, C, CO>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ext_context(self, context: CO) -> ErrorReporter<E, C>;
}

impl<E, C, CO> IntoErrorReporter<E, C, CO> for E
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

impl<C, CO, E> IntoErrorReporter<E, C, CO> for ErrorReporter<E, C>
where
    E: std::error::Error + Send + Sync + 'static,
    C: ErrorContext<CO>,
{
    fn ext_context(mut self, context: CO) -> ErrorReporter<E, C> {
        self.context.push(context);
        self
    }
}

/// Helper struct for efficiently numbering and correctly indenting multi line display
/// implementations
pub struct Indented<'a, D> {
    inner: &'a mut D,
    ind: Option<usize>,
    started: bool,
}

impl<'a, D> Indented<'a, D> {
    /// Wrap a formatter number the first line and indent all lines of input before forwarding the
    /// output to the inner formatter
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
                line = line.trim_start();
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
