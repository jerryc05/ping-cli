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

// impl<T: Into<Cow<'a, str>>> From<(T, &'static str, u32)> for MyErr<'a> {
//   fn from(arg: (T, &'static str, u32)) -> Self {
//     Self { err: arg.0.into(), file: arg.1, line: arg.2 }
//   }
// }

impl<T: Debug> From<(&T, &'static str, u32)> for MyErr {
  fn from(arg: (&T, &'static str, u32)) -> Self {
    Self { err: format!("{:?}", arg.0), file: arg.1, line: arg.2 }
  }
}

impl Debug for MyErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    const ERROR: &str = "ERROR";
    writeln!(f, "
{1:-^0$}
msg: [{2:?}]

at: [{3}:{4}]
{1:-^0$}",
             CONSOLE_FMT_WIDTH, ERROR, self.err, self.file, self.line)
  }
}