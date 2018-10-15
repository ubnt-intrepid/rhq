use std::fmt::Arguments;
use std::io::{self, Write};

#[derive(Default)]
pub struct Printer {
    pub(crate) verbose: bool,
}

impl Printer {
    pub fn print(&self, args: Arguments) {
        if self.verbose {
            let stdout = io::stdout();
            let _ = stdout.lock().write_fmt(args);
        }
    }
}
