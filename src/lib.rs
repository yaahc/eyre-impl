#![feature(backtrace)]

use core::fmt::Write;
pub use kvfmt::kvfmt;
use std::fmt;

#[derive(Debug)]
pub struct HumanErr {
    msg: String,
    note: Option<String>,
    dbg: Option<String>,
}

impl HumanErr {
    pub fn msg(msg: &'static str) -> Self {
        Self {
            msg: msg.into(),
            note: None,
            dbg: None,
        }
    }

    pub fn note(self, note: &'static str) -> Self {
        let Self { msg, dbg, .. } = self;

        Self {
            msg,
            note: Some(note.into()),
            dbg,
        }
    }

    pub fn debug(self, dbg: &'static str) -> Self {
        let Self { msg, note, .. } = self;

        Self {
            msg,
            note,
            dbg: Some(dbg.into()),
        }
    }

    pub fn with_debug<T>(self, dbg: T) -> Self
    where
        T: FnOnce() -> String,
    {
        let Self { msg, note, .. } = self;

        Self {
            msg,
            note,
            dbg: Some(dbg()),
        }
    }
}

impl fmt::Display for HumanErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;

        if let Some(note) = &self.note {
            writeln!(f, "\n\nNote:")?;
            write!(Indented::new(f), "{}", note)?;
        }

        if let Some(dbg) = &self.dbg {
            writeln!(f, "\n\nDebug:")?;
            write!(Indented::new(f), "{}", dbg)?;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        $crate::HumanErr::msg($msg)
    };

    ($msg:expr $(, $($args:tt)*)?) => {
        $crate::HumanErr::msg($msg).with_debug(|| $crate::kvfmt!( $($($args)*)? ))
    };
}

struct Indented<'a, D> {
    inner: &'a mut D,
    started: bool,
}

impl<'a, D> Indented<'a, D> {
    fn new(inner: &'a mut D) -> Self {
        Self {
            inner,
            started: false,
        }
    }
}

impl<T> Write for Indented<'_, T>
where
    T: Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (ind, line) in s.split('\n').enumerate() {
            if !self.started {
                self.started = true;
                self.inner.write_fmt(format_args!("    "))?;
            } else if ind > 0 {
                self.inner.write_char('\n')?;
                self.inner.write_fmt(format_args!("    "))?;
            }

            self.inner.write_fmt(format_args!("{}", line))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let local: u32 = 42;
        let msg = "Failed to get root of the current git repository";
        let note = "gsync can only be run from within git repositories";
        let dbg = || format!("local={:?}", local);

        let err = HumanErr::msg(msg).note(note).with_debug(dbg);

        let expected = format!("{}\n\nNote:\n    {}\n\nDebug:\n    {}", msg, note, dbg());

        assert_eq!(expected, err.to_string());
    }

    #[test]
    fn err() {
        let local: u32 = 42;
        let msg = "Failed to get root of the current git repository";
        let note = "gsync can only be run from within git repositories";
        let dbg = || format!("local={:?}", local);

        let err = err!(msg, ?local).note(note);

        let expected = format!("{}\n\nNote:\n    {}\n\nDebug:\n    {}", msg, note, dbg());

        assert_eq!(expected, err.to_string());
    }

    #[test]
    fn no_digits() {
        let input = "verify\nthis";
        let expected = "    verify\n    this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }
}
