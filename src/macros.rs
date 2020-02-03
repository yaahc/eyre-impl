#[macro_export]
macro_rules! wrap {
    ($($arg:tt)*) => {
        |err| $crate::ErrReport::from($crate::private::wrap!($($arg)*)(err))
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::ErrReport::from($crate::private::err!($($arg)*))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $msg:literal) => {
        if !$cond {
            return $crate::private::Err($crate::err!($msg));
        }
    };
    ($cond:expr, $fmt:literal, $($arg:tt)*) => {
        if !$cond {
            return $crate::private::Err($crate::err!($fmt, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($msg:literal) => {
        return $crate::private::Err($crate::err!($msg));
    };
    ($fmt:literal, $($arg:tt)*) => {
        return $crate::private::Err($crate::err!($fmt, $($arg)*));
    };
}

#[cfg(test)]
mod tests {
    fn try_code_anyhow() -> Result<(), crate::ErrReport> {
        let code = 1;

        ensure!(code == 0, "Command exited with a non zero status code");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn ensure_coerce() {
        try_code_anyhow().unwrap();
    }
}
