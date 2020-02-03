#![feature(backtrace)]
mod chain;
mod context;
mod macros;
mod report;

pub use context::ContextExt;
pub use err as format_err;
pub use report::ErrReport;
pub use report::RootCauseFirst;
pub use report::RootCauseLast;

pub type Result<T> = std::result::Result<T, ErrReport>;

#[doc(hidden)]
pub mod private {
    pub use adhocerr::err;
    pub use adhocerr::wrap;
    pub use core::result::Result::Err;
}
