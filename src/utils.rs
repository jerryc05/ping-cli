#![allow(dead_code)]

#[inline]
pub(crate) const fn is_debug() -> bool {
  cfg!(debug_assertions)
}