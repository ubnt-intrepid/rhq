pub mod process;

use std::borrow::{Borrow, Cow};
use std::path::PathBuf;
use shellexpand;


pub fn make_path_buf<S: AsRef<str>>(s: S) -> ::Result<PathBuf> {
  shellexpand::full(s.as_ref()).map(|s| PathBuf::from(s.borrow() as &str)).map_err(Into::into)
}
