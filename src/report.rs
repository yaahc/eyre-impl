use crate::chain::Chain;
use crate::context::{ContextObj, ContextObject};
#[cfg(backtrace)]
use std::backtrace::Backtrace;
use std::fmt;
use std::marker::PhantomData;
use strategy::ErrorInfo;

pub struct RootCauseFirst<C>(PhantomData<C>);
pub struct RootCauseLast<C>(PhantomData<C>);

pub struct ErrReport<C = BothTrace, S = RootCauseFirst<C>>(pub(crate) Box<ReportImpl<C, S>>)
where
    C: Default + fmt::Display,
    S: strategy::ErrorFormatter<C>;

impl<C, S> ErrReport<C, S>
where
    C: Default + fmt::Display,
    S: strategy::ErrorFormatter<C>,
{
    pub fn note(&mut self, context: impl ContextObj) {
        self.0.context.push(ContextObject::Note(Box::new(context)));
    }

    pub fn warn(&mut self, context: impl ContextObj) {
        self.0.context.push(ContextObject::Warn(Box::new(context)));
    }
}

impl<C, S> fmt::Debug for ErrReport<C, S>
where
    C: Default + fmt::Display,
    S: strategy::ErrorFormatter<C>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.debug(f)
    }
}

impl<C, S> fmt::Display for ErrReport<C, S>
where
    C: Default + fmt::Display,
    S: strategy::ErrorFormatter<C>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.display(f)
    }
}

impl<E, C, S> From<E> for ErrReport<C, S>
where
    C: Default + fmt::Display,
    S: strategy::ErrorFormatter<C>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        ErrReport(Box::new(ReportImpl {
            error: Box::new(err),
            contextt: C::default(),
            context: Vec::new(),
            phantom: PhantomData,
        }))
    }
}

mod strategy;

pub struct ReportImpl<C, S> {
    error: Box<dyn std::error::Error + Send + Sync + 'static>,
    pub(crate) contextt: C,
    pub(crate) context: Vec<ContextObject>,
    phantom: PhantomData<S>,
}

pub struct BothTrace {
    #[cfg(backtrace)]
    backtrace: Backtrace,
    pub(crate) span_backtrace: tracing_error::SpanTrace,
}

impl Default for BothTrace {
    fn default() -> Self {
        Self {
            #[cfg(backtrace)]
            backtrace: std::backtrace::Backtrace::capture(),
            span_backtrace: tracing_error::SpanTrace::capture(),
        }
    }
}

impl fmt::Display for BothTrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.span_backtrace)?;

        #[cfg(backtrace)]
        {
            use std::backtrace::BacktraceStatus;

            let backtrace = &self.backtrace;
            if let BacktraceStatus::Captured = backtrace.status() {
                let mut backtrace = backtrace.to_string();
                if backtrace.starts_with("stack backtrace:") {
                    // Capitalize to match "Caused by:"
                    backtrace.replace_range(0..7, "Stack B");
                }
                backtrace.truncate(backtrace.trim_end().len());
                write!(f, "\n\n{}", backtrace)?;
            }
        }

        Ok(())
    }
}

impl<C, S> ReportImpl<C, S> {
    fn error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self.error.as_ref()
    }

    pub(crate) fn chain(&self) -> Chain {
        Chain::new(self.error())
    }
}

impl<C, S> ReportImpl<C, S>
where
    C: Default + fmt::Display,
    S: strategy::ErrorFormatter<C>,
{
    pub(crate) fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error())?;

        if f.alternate() {
            for cause in self.chain().skip(1) {
                write!(f, ": {}", cause)?;
            }
        }

        Ok(())
    }

    pub(crate) fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = self.error();

        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        S::fmt_error(
            ErrorInfo {
                error,
                context: &self.contextt,
            },
            f,
        )
    }
}
