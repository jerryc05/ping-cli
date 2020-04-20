#![allow(dead_code)]

use std::fmt::{Debug, Formatter, Result};

pub(crate) const CONSOLE_FMT_WIDTH: usize = 50;

#[inline]
pub(crate) const fn is_debug() -> bool {
  cfg!(debug_assertions)
}

pub struct MyErr {
  err: String,
  file: &'static str,
  line: u32,
}

impl MyErr {
  pub fn from_str<T: Into<String>>(
    str: T, file: &'static str, line: u32,
  ) -> Self {
    Self { err: str.into(), file, line }
  }

  pub fn from_err<T: Debug>(
    err: &T, file: &'static str, line: u32,
  ) -> Self {
    Self { err: format!("{:?}", err), file, line }
  }
}

impl Debug for MyErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    const ERROR: &str = "ERROR";
    writeln!(f, "
{1:-^0$}
| msg: [{2}]
|
| at: [{3}:{4}]
{1:-^0$}",
             CONSOLE_FMT_WIDTH, ERROR, self.err, self.file, self.line)
  }
}