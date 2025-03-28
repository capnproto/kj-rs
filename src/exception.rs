// Copied from https://github.com/dtolnay/cxx/blob/d2727ef4a665cd07140b3662624a43a5cb56e99a/src/exception.rs
//
// The original `cxx` crate is dual-licensed under MIT and Apache 2.0 licenses. You will find a copy
// of one of those licenses in the file named LICENSE in this repository's root directory.
//
// Slightly modified to fit into kj-rs.

use core::fmt::{self, Display};
use std::error::Error;

/// Exception thrown from an `extern "C++"` function.
#[derive(Debug)]
pub struct Exception {
    pub(crate) what: Box<str>,
}

impl Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.what)
    }
}

impl Error for Exception {}

impl Exception {
    #[allow(missing_docs)]
    pub fn what(&self) -> &str {
        &self.what
    }
}