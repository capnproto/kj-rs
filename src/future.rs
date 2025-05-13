// This file contains boilerplate which must occur once per crate, rather than once per type.

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use crate::KjWaker;

// NOTE: FuturePollStatus must be kept in sync with the C++ enum of the same name in future.h
// Ideally, this would live in kj-rs's `crate::ffi` module, and code which depends on kj-rs would be
// able to include `kj-rs/src/lib.rs.h`. I couldn't figure out how to expose that generated lib.rs.h
// header to Bazel dependents, though, so I'm just splatting it here.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct FuturePollStatus {
    pub repr: u8,
}

#[allow(non_upper_case_globals)]
impl FuturePollStatus {
    pub const Pending: Self = Self { repr: 0 };
    pub const Complete: Self = Self { repr: 1 };
    pub const Error: Self = Self { repr: 2 };
}

// Based on `cxx-async`'s version:
// https://github.com/pcwalton/cxx-async/blob/ac98030dd6e5090d227e7fadca13ec3e4b4e7be7/cxx-async/src/lib.rs#L824
//
// C++ calls this to poll a wrapped Rust future.
//
// SAFETY:
// * This is a low-level function called by our C++ code.
// * `Pin<&mut Future>` is marked `#[repr(transparent)]`, so it's FFI-safe.
// * We catch all panics inside `poll` so that they don't unwind into C++.
#[doc(hidden)]
pub unsafe extern "C" fn box_future_poll<F, T, E>(
    future: Pin<&mut F>,
    waker: &KjWaker,
    result: *mut (),
) -> FuturePollStatus
where
    F: std::future::Future<Output = std::result::Result<T, E>>,
    T: Unpin,
    E: std::error::Error,
{
    let waker = Waker::from(waker);
    let mut context = Context::from_waker(&waker);
    // let result = panic::catch_unwind(AssertUnwindSafe(move || {
    match future.poll(&mut context) {
        Poll::Ready(Ok(value)) => {
            unsafe { std::ptr::write(result as *mut T, value) };
            FuturePollStatus::Complete
        }
        Poll::Ready(Err(error)) => {
            unsafe { std::ptr::write(result as *mut String, error.to_string()) };
            FuturePollStatus::Error
        }
        Poll::Pending => FuturePollStatus::Pending,
    }
    // }));

    // match result {
    //     Ok(result) => result,
    //     Err(error) => {
    //         drop(writeln!(
    //             io::stderr(),
    //             "Rust async code panicked when awaited from C++: {:?}",
    //             error
    //         ));
    //         process::abort();
    //     }
    // }
}

// Expose Pin<Box<dyn Future<Output = ()>> to C++ as BoxFutureVoid.
//
// We want to allow C++ to own Rust Futures in a Box. At present, cxx-rs can easily expose Box<T>
// directly to C++ only if T implements Sized and Unpin. Dynamic trait types like `dyn Future` don't
// meet these requirements. One workaround is to pass Box<Box<dyn Future>> around. With a few more
// lines of boilerplate, we can avoid the extra Box:, as dtolnay showed in this demo PR:
// https://github.com/dtolnay/cxx/pull/672/files

pub struct BoxFuture<T>(Pin<Box<dyn Future<Output = T> + Send>>);

impl<T> Future for BoxFuture<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<T> {
        self.0.as_mut().poll(cx)
    }
}

// A From implementation to make it easier to convert from an arbitrary Future
// type into a BoxFuture<T>.
//
// Of interest: the `async-trait` crate contains a macro which seems like it could eliminate the
// `Box::pin(f).into()` boilerplate this currently requires. https://github.com/dtolnay/async-trait
//
// TODO(now): Understand why 'static is needed.
impl<T, F: Future<Output = T> + Send + 'static> From<Pin<Box<F>>> for BoxFuture<T> {
    fn from(value: Pin<Box<F>>) -> Self {
        BoxFuture(value)
    }
}
