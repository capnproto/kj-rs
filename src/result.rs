// Copied from https://github.com/dtolnay/cxx/blob/d2727ef4a665cd07140b3662624a43a5cb56e99a/src/result.rs
//
// The original `cxx` crate is dual-licensed under MIT and Apache 2.0 licenses. You will find a copy
// of one of those licenses in the file named LICENSE in this repository's root directory.
//
// Slightly modified to fit into kj-rs.

#![allow(missing_docs)]

use crate::exception::Exception;
use core::fmt::Display;
use core::ptr::{self, NonNull};
use core::result::Result as StdResult;
use core::slice;
use core::str;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct PtrLen {
    pub ptr: NonNull<u8>,
    pub len: usize,
}

#[repr(C)]
pub union Result {
    err: PtrLen,
    ok: *const u8, // null
}

impl Result {
    pub unsafe fn exception(self) -> StdResult<(), Exception> {
        unsafe {
            if self.ok.is_null() {
                Ok(())
            } else {
                let err = self.err;
                let slice = slice::from_raw_parts_mut(err.ptr.as_ptr(), err.len);
                let s = str::from_utf8_unchecked_mut(slice);
                Err(Exception {
                    what: Box::from_raw(s),
                })
            }
        }
    }
}

// =======================================================================================

// Copied from https://github.com/dtolnay/cxx/blob/d2727ef4a665cd07140b3662624a43a5cb56e99a/src/symbols/exception.rs
//
// The original `cxx` crate is dual-licensed under MIT and Apache 2.0 licenses. You will find a copy
// of one of those licenses in the file named LICENSE in this repository's root directory.
//
// Slightly modified to fit into kj-rs.

#[export_name = "kjrs$exception"]
unsafe extern "C" fn exception(ptr: *const u8, len: usize) -> PtrLen {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let string = String::from_utf8_lossy(slice);
    let len = string.len();
    let raw_str = Box::into_raw(string.into_owned().into_boxed_str());
    let raw_u8 = raw_str.cast::<u8>();
    let nonnull = unsafe { NonNull::new_unchecked(raw_u8) };
    PtrLen { ptr: nonnull, len }
}
